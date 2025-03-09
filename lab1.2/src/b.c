#include <math.h>
#include <stdio.h>

int main(void) {
  int op_counter;

  int n;
  scanf("%d", &n);

  double result = 0;
  double last_numerator = 1;
  op_counter = 3; // init of `n`, `result`, `last_numerator`
  for (int i = 1; i <= n; i++) {
    // `i` init or increment, `i <= n` check, jump
    op_counter += 3;

    last_numerator *= log(i + 2); // multiplication, log call, addition
    op_counter += 3;

    result += last_numerator / (3 - sin(i) * sin(i));
    op_counter +=
        6; // addition, division, subtraction, sin call (x2), multiplication
  }

  printf("Result: %.7lf\n", result);
  printf("Operations counter: %d\n", op_counter);
}
