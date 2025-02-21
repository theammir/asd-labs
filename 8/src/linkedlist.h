#pragma once

#include <stdint.h>

typedef struct l_list {
    int32_t value;
    struct l_list *next;
} l_list;

l_list *l_list_new(const int32_t value);
l_list *l_list_push(l_list *list, const int32_t value);
void l_list_display(l_list *list);
void l_list_free(l_list *list);

