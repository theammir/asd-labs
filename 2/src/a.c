#include <math.h>
#include <stdio.h>

int main(void) {
	// Operations are: variable assignment, boolean logic, math operations, external math functions,
	// jumps to the next loop iteration
	int op_counter;
	
	int n;
	scanf("%d", &n);
	
	double result = 0;
	op_counter = 2; // init of `n`, `result`
	for (int i = 1; i <= n; i++) {
		// `i` init or increment, `i <= n` check, jump
		op_counter += 3;

		double numerator = 1;
		op_counter += 1; // init of `numerator`

		for (int j = 1; j <= i; j++) {
			// `j` init or increment, `j <= i` check, jump
			op_counter += 3;

			numerator *= log(j + 2); // multiplication, log call, addition
			op_counter += 3;
		}
		// +final `j <= i` check, -unneeded jump on the first iteration
		
		// addition, division, subtraction, sin call (x2), multiplication
		result += numerator / (3 - sin(i) * sin(i));
		op_counter += 6; 
	}
	// +final `i <= n` check, -unneeded jump on the first iteration (<- comment omitted in b.c)

	printf("Result: %.7lf\n", result);
	printf("Operations counter: %d\n", op_counter);
}
