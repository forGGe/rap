#include "slist.h"

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

void export_slist_init(struct slist_node *head) {
    slist_init(head);
}

static struct slist_exemplar *slist_exemplar_get(struct slist_node *node) {
    struct slist_exemplar *s = slist_container(node, struct slist_exemplar, node);
    return s;
}

void slist_exemplar_alloc(struct slist_exemplar *head, unsigned value) {
    struct slist_exemplar *node = malloc(sizeof(*node));
    assert(node != NULL);

    node->value = value;
    slist_link(&head->node, &node->node);
}

void slist_exemplar_dealloc(struct slist_exemplar *node) {
    /* Find an item right before the target one */
    struct slist_node *prev = &node->node;
    while (prev->next != &node->node) {
        prev = prev->next;
    }

    slist_unlink(prev, &node->node);
    free(node);
}

struct slist_exemplar *slist_exemplar_next(struct slist_exemplar *node) {
    return slist_exemplar_get(node->node.next);
}
