#include "input.h"
#include <stdio.h>

typedef struct {
  double term;
  double sum;
} Result;

Result sum_nth_asc(unsigned int i, double x) {
  Result result;
  if (i == 0) {
    result.term = 1.0;
    result.sum = 1.0;
    printf("i = 0: %lf\n", result.term);

    return result;
  } else {
    Result previous = sum_nth_asc(i - 1, x);
    result.term = previous.term * x * x / (4 * i * i - 2 * i);
    result.sum = previous.sum + result.term;
    printf("i = %u: %lf\n", i, result.term);

    return result;
  }
}

int main(int argc, char **argv) {
  unsigned int n;
  double x;

  get_input(argc, argv, &n, &x);

  if (n == 0) {
    printf("ch(%lf) = 0\n", x);
    return 0;
  }

  Result result = sum_nth_asc(n - 1, x);
  printf("ch(%lf) = %lf\n", x, result.sum);
  return 0;
}
