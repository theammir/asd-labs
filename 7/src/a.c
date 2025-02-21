#include "input.h"
#include <stdio.h>

double sum_nth_desc(const unsigned int n, const double x, unsigned int i,
                    double previous, double sum) {
  if (i >= n) {
    return sum;
  }

  double current;
  if (i == 0) {
    current = 1.0;
  } else {
    current = previous * x * x / (4 * i * i - 2 * i);
  }
  printf("i = %u: %lf\n", i, current);

  sum += current;

  return sum_nth_desc(n, x, i + 1, current, sum);
}

int main(int argc, char **argv) {
  unsigned int n;
  double x;

  get_input(argc, argv, &n, &x);

  double sum = sum_nth_desc(n, x, 0, 0, 0);

  printf("ch(%lf) = %lf\n", x, sum);
  return 0;
}
