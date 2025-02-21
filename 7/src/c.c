#include "input.h"
#include <stdio.h>

double sum_nth_mixed(unsigned int i, unsigned int n, double x, double prev) {
  double current;
  if (i == 0) {
    current = 1.0;
  } else {
    current = prev * x * x / (4 * i * i - 2 * i);
  }
  printf("i = %u: %lf\n", i, current);

  if (i >= n - 1) {
    return current;
  }

  double sum_rest = sum_nth_mixed(i + 1, n, x, current);
  return current + sum_rest;
}

int main(int argc, char **argv) {
  unsigned int n;
  double x;
  get_input(argc, argv, &n, &x);

  if (n == 0) {
    printf("ch(%lf) = 0\n", x);
    return 0;
  }

  double total = sum_nth_mixed(0, n, x, 0);
  printf("ch(%lf) = %lf\n", x, total);
  return 0;
}
