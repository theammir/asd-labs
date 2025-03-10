#include "input.h"
#include "linkedlist.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

typedef struct {
  LinkedList *head;
  LinkedList *tail;
} Group;

Group rearrange_group(LinkedList *head) {
  LinkedList *odd_head = head, *even_head = head->next;
  LinkedList *odd_tail = odd_head, *even_tail = even_head;

  for (uint32_t i = 0; i < GROUP_SIZE / 2 - 1; i++) {
    LinkedList *next_odd = even_tail->next;
    LinkedList *next_even = next_odd->next;

    odd_tail->next = next_odd;
    even_tail->next = next_even;

    odd_tail = next_odd;
    even_tail = next_even;
  }

  if (odd_head->value >= 0) {
    odd_tail->next = even_tail->next;
    even_tail->next = odd_head;
    return (Group){even_head, odd_tail};
  } else {
    odd_tail->next = even_head;
    return (Group){odd_head, even_tail};
  }
}

int main(int32_t argc, char **argv) {
  srand(clock());

  LinkedList *list = get_input(argc, argv);
  LinkedList_display(list);

  printf("Sorting.\n");

  Group first = rearrange_group(list);
  Group current = first;
  while (current.tail->next != NULL) {
    Group next = rearrange_group(current.tail->next);
    current.tail->next = next.head;
    current = next;
  }

  LinkedList_display(first.head);

  LinkedList_free(first.head);
  return 0;
}
