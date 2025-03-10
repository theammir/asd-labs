#include "linkedlist.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

LinkedList *LinkedList_new(const int32_t value) {
  LinkedList *head = (LinkedList *)malloc(sizeof(LinkedList));
  head->value = value;
  head->next = (struct LinkedList *)NULL;
  return head;
}

LinkedList *LinkedList_push(LinkedList *list, const int32_t value) {
  LinkedList *next = LinkedList_new(value);
  list->next = (struct LinkedList *)next;
  return next;
}

void LinkedList_display(LinkedList *list) {
  LinkedList *node = list;
  size_t index = 0;
  while (node != NULL) {
    printf("%d ", node->value);
    index++;
    // NOTE: should made the output clearer, but isn't suitable for LaTeX
    //
    // if (index % 20 == 0) {
    //   printf("\033[0m");
    // } else if (index % 10 == 0) {
    //   printf("\033[38;5;245m");
    // }
    node = node->next;
  }
  // printf("\033[0m");
  printf("\n");
}

void LinkedList_free(LinkedList *list) {
  if (list == NULL) {
    return;
  }
  LinkedList_free(list->next);
  free(list);
}
