#include "input.h"
#include "linkedlist.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

l_list *get_input(const int32_t argc, char **argv) {
  l_list *head = l_list_new(0);
  l_list *tail = head;

  if (argc <= 1) {
    printf("No input received. Generating random sequence...\n");
    int8_t sign = (rand() % 2) ? 1 : -1;
    for (int32_t i = (rand() % 5 + 1) * GROUP_SIZE; i > 0; i--) {
      int32_t element = rand() % 100;
      tail = l_list_push(tail, sign * element);
      sign *= -1;
    }
  } else {
    if ((argc - 1) % GROUP_SIZE != 0) {
      printf("The amount of elements has to be a multiple of %d!", GROUP_SIZE);
      return NULL;
    }
    // TODO: sign validation
    for (int32_t i = 1; i < argc; i++) {
      tail = l_list_push(tail, atoi(argv[i]));
    }
  }
  return head->next;
}
