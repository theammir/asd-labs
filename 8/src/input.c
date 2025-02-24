#include "input.h"
#include "linkedlist.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

LinkedList *get_input(const int32_t argc, char **argv) {
  LinkedList *head = LinkedList_new(0);
  LinkedList *tail = head;

  if (argc <= 1) {
    printf("No input received. Generating random sequence...\n");
    int8_t sign = (rand() % 2) ? 1 : -1;
    for (int32_t i = (rand() % 5 + 1) * GROUP_SIZE; i > 0; i--) {
      int32_t element = rand() % 100;
      tail = LinkedList_push(tail, sign * element);
      sign *= -1;
    }
  } else {
    if ((argc - 1) % GROUP_SIZE != 0) {
      printf("The amount of elements has to be a multiple of %d!", GROUP_SIZE);
      return NULL;
    }
    // TODO: sign validation
    for (int32_t i = 1; i < argc; i++) {
      tail = LinkedList_push(tail, atoi(argv[i]));
    }
  }
  return head->next;
}
