#pragma once

#include <stdint.h>
#include <stddef.h>

// Circular single-linked list,
struct slist_node {
    struct slist_node *next;
};

static inline void slist_init(struct slist_node *head) {
    head->next = head;
}

static inline void slist_link(struct slist_node *head, struct slist_node *next) {
    next->next = head->next;
    head->next = next;
}

static inline struct slist_node *slist_unlink(struct slist_node *prev, struct slist_node *curr) {
   prev->next = curr->next;
   curr->next = curr;
   return curr;
}

#define slist_container(p, t, m) \
    ((t *)((uint8_t *)(p) - offsetof(t, m)))

// -----------------------------------------------------------------------------

// To aid Rust inline export
void export_slist_init(struct slist_node *head);

// -----------------------------------------------------------------------------

struct slist_exemplar {
    struct slist_node node;
    unsigned value;
};

void slist_exemplar_alloc(struct slist_exemplar *head, unsigned value);
void slist_exemplar_dealloc(struct slist_exemplar *node);
struct slist_exemplar *slist_exemplar_next(struct slist_exemplar *node);
