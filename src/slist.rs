use std::cell::Cell;

struct Node<'n, T: Sized> {
    next: Cell<Option<&'n Node<'n, T>>>,
    v: T,
}

impl<'n, T: Sized> Node<'n, T> {
    fn new(v: T) -> Self {
        Node {
            v,
            next: Cell::new(None),
        }
    }

    fn link(&self, next: &'n Self) {
        self.next.set(Some(next));
    }
}

struct NodeIter<'n, T: Sized> {
    curr: Cell<Option<&'n Node<'n, T>>>,
}

impl<'n, T> Iterator for NodeIter<'n, T> {
    type Item = &'n T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(it) = self.curr.get() else {
            return None;
        };

        self.curr.set(it.next.get());
        return Some(&it.v);
    }
}

impl<'n, T> IntoIterator for &'n Node<'n, T> {
    type Item = &'n T;
    type IntoIter = NodeIter<'n, T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            curr: Cell::new(Some(self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::Node;

    #[test]
    fn test_reverse_list() {
        let head = Node::new(1);

        let n1 = Node::new(77);
        let n2 = Node::new(23);
        let n3 = Node::new(4);

        head.link(&n1);
        n1.link(&n2);
        n2.link(&n3);

        for (i, v) in head.into_iter().enumerate() {
            println!("{} -> {}", i, v);
        }
    }

    #[test]
    fn test_link_all_nodes() {
        let head = Cell::new(None);
        let ns = [Node::new(5), Node::new(42), Node::new(11)];

        {
            let ns0 = &ns[0];
            let ns1 = &ns[1];
            let ns2 = &ns[2];
            ns0.link(ns1);
            ns1.link(ns2);
            head.set(Some(ns0));
        }

        let mut it = &head;

        while let Some(n) = it.get() {
            println!("{}", n.v);
            it = &n.next;
        }
    }
}