// обов'язкові логічні операції, не мають бути надлишковиим
// y = -9x^3 + 5x^2		(2; 12] U (22, 32)
//   = -x^2 - 12		(-inf, 0]
#include <stdio.h>

int main(void) {
  float x, y;

  scanf("%f", &x);

  if (x <= 0) {
    y = -x * x - 12;
  } else if (x <= 2 || (x > 12 && x <= 22) || (x >= 32)) {
    printf("no such value exists\n");
    return 0;
  } else {
    y = -9 * x * x * x + 5 * x * x;
  }

  printf("%f\n", y);
  return 0;
}
