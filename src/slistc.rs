use std::{
    marker::PhantomData,
    pin::{pin, Pin},
};

use foreign::slist_exemplar_dealloc;

mod foreign {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/slistc_bindings.rs"));
}

struct Intrusive<'i> {
    // Head has a special meaning since the list is circular
    // We can't move it anywhere as list items pointers will be invalidated
    h: Pin<Box<foreign::slist_exemplar>>,
    // Phantom data to make sure all associated refs are bound to this container
    _p: PhantomData<&'i ()>,
}

impl<'i> Intrusive<'i> {
    fn new() -> Self {
        let h = Box::pin(foreign::slist_exemplar {
            node: foreign::slist_node {
                next: std::ptr::null_mut(),
            },
            value: 0,
        });

        let mut s = Self { _p: PhantomData, h };

        unsafe {
            foreign::export_slist_init(&mut s.h.node);
        }

        s
    }

    fn insert(&mut self, v: u32) {
        let h = &mut *self.h;
        unsafe {
            foreign::slist_exemplar_alloc(h, v);
        }
    }
}

impl<'i> Drop for Intrusive<'i> {
    fn drop(&mut self) {
        let h = &mut *self.h;

        loop {
            let next = unsafe { foreign::slist_exemplar_next(h) };

            // Don't touch head since it has a special meaning
            if next as *const _ == h as *const _ {
                break;
            }

            unsafe {
                slist_exemplar_dealloc(next);
            }
        }
    }
}

struct IntrusiveIter<'i> {
    head: Intrusive<'i>,
    curr: &'i mut foreign::slist_exemplar,
}

impl<'i> Iterator for IntrusiveIter<'i> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let h = &mut *self.head.h;

        if h as *const _ == self.curr as *const _ {
            return None;
        }

        let v = self.curr.value;

        self.curr = unsafe { &mut *foreign::slist_exemplar_next(self.curr) };

        Some(v)
    }
}

impl<'i> IntoIterator for Intrusive<'i> {
    type Item = u32;
    type IntoIter = IntrusiveIter<'i>;

    fn into_iter(mut self) -> Self::IntoIter {
        let h = &mut *self.h;

        let next = unsafe { &mut *foreign::slist_exemplar_next(h) };

        Self::IntoIter {
            head: self,
            curr: next,
        }
    }
}

impl<'i> FromIterator<u32> for Intrusive<'i> {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut lst = Intrusive::new();

        for it in iter {
            lst.insert(it);
        }

        lst
    }
}

#[cfg(test)]
mod tests {
    use super::Intrusive;

    #[test]
    fn test_simple_insert() {
        let mut v = vec![55, 22, 1, 42, 898, 101, 45, 55, 88];
        let mut slst = Intrusive::new();

        for it in &v {
            slst.insert(*it);
        }

        let rv: Vec<u32> = slst.into_iter().collect();

        // We know that adding to this list reverses the order of elements
        v.reverse();
        assert_eq!(v, rv);
    }

    #[test]
    fn test_from_iterator() {
        let mut v = vec![42, 22, 1, 42, 898, 101, 45, 55, 88];

        let slst: Intrusive = v.iter().map(|&x| x as u32).collect();
        let rv: Vec<u32> = slst.into_iter().collect();

        // We know that adding to this list reverses the order of elements
        v.reverse();
        assert_eq!(v, rv);
    }
}
