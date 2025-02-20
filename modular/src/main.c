#include <stdio.h>
#include <stdlib.h>

double sin_approx(double x, int n) {
  double result = 0;
  double last_x = x;
  double last_fact = 1;

  for (int i = 1; i <= n; i++) {
    if (i > 1) {
      last_x *= x * x;
      last_fact *= (2 * i - 2) * (2 * i - 1);
    }
    double part = last_x / last_fact;
    // i would imagine a conditional still being faster
    // than a for loop
    part *= (i % 2 == 0) ? -1 : 1;

    result += part;
  }

  return result;
}

int main(int argc, char **argv) {
  int n;
  double x;

  if (argc <= 1) {
    printf("\033[2mYou can use the program as a CLI: \033[1;2m./main [n] "
           "[x]\n\033[0m\n\n");
    printf("Enter precision    (n): ");
    scanf("%d", &n);

    printf("Enter sin argument (x): ");
    scanf("%lf", &x);
  } else if (argc == 2) {
    n = atoi(argv[1]);

    printf("Enter sin argument (x): ");
    scanf("%lf", &x);
  } else {
    n = atoi(argv[1]);
    x = atof(argv[2]);
  }

  printf("sin(%lf) ≈ %lf\n", x, sin_approx(x, n));
  return 0;
}

// vim: ts=4: sw=4
