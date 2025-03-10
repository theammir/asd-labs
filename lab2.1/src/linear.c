#include "input.h"
#include <stdio.h>

double series_next(unsigned int i, double x, double previous) {
  double result;
  if (i == 0) {
    result = 1;
  } else {
    result = previous * x * x / (4 * i * i - 2 * i);
  }
  printf("i = %u: %lf\n", i, result);
  return result;
}

int main(int argc, char **argv) {
  unsigned int n;
  double x;

  get_input(argc, argv, &n, &x);

  double sum = 0;
  double previous = 0;
  for (unsigned int i = 1; i <= n; i++) {
    previous = series_next(i - 1, x, previous);
    sum += previous;
  }
  printf("ch(%lf) = %lf\n", x, sum);
  return 0;
}
