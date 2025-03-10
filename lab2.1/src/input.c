#include <stdio.h>
#include <stdlib.h>

void get_input(const int argc, char **argv, unsigned int *n, double *x) {
  if (argc <= 1) {
    printf("\033[2mYou can use the program as a CLI: \033[1;2m%s [n] "
           "[x]\n\033[0m\n\n",
           argv[0]);
    printf("Enter precision (n): ");
    scanf("%u", n);

    printf("Enter argument  (x): ");
    scanf("%lf", x);
  } else if (argc == 2) {
    *n = atoi(argv[1]);

    printf("Enter argument (x): ");
    scanf("%lf", x);
  } else {
    *n = atoi(argv[1]);
    *x = atof(argv[2]);
  }
}
