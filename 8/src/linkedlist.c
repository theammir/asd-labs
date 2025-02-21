#include "linkedlist.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

l_list *l_list_new(const int32_t value) {
  l_list *head = (l_list *)malloc(sizeof(l_list));
  head->value = value;
  head->next = (struct l_list *)NULL;
  return head;
}

l_list *l_list_push(l_list *list, const int32_t value) {
  l_list *next = l_list_new(value);
  list->next = (struct l_list *)next;
  return next;
}

void l_list_display(l_list *list) {
  l_list *node = list;
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

void l_list_free(l_list *list) {
  if (list == NULL) {
    return;
  }
  l_list_free(list->next);
  free(list);
}
