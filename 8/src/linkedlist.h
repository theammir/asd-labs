#pragma once

#include <stdint.h>

typedef struct LinkedList {
    int32_t value;
    struct LinkedList *next;
} LinkedList;

LinkedList *LinkedList_new(const int32_t value);
LinkedList *LinkedList_push(LinkedList *list, const int32_t value);
void LinkedList_display(LinkedList *list);
void LinkedList_free(LinkedList *list);

