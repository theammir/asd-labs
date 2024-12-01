// F_1           = x / (0.525 + 0.5x)^2
// F_(i+1)       = F_i * F_1(3 - 2i)/(2i), i > 0
// sum 1..=n F_i = sqrt(x)               , 0.5 < x < 1 - еталон?
#include <stdio.h>

const double X = 0.7;

double calculate_sum_down(unsigned int i, double f, double sum) {
	double next_element(unsigned int i, double sum) {
		if (i <= 1) {
			return sum;
		}
	}
	sum += calculate_sum_down(--i, f, sum);
}

int main(void) {
	unsigned int n;
	double t;
	printf("Enter n: ");
	scanf("%ud", &n);

	double f_1 = X / (t = 0.525 + 0.5*X);
	f_1 /= t;
	f_1--;

	return 0;
}

// vim: sw=4: ts=4
