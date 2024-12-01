// одиничні операції порівняння без логічних операцій
// y = -9x^3 + 5x^2		(2; 12] U (22, 32)
//   = -x^2 - 12		(-inf, 0]
#include <stdio.h>

float f1(float x) {
	return -9 * x*x*x + 5 * x*x;
}

float f2(float x) {
	return -x*x - 12;
}

int main(void) {
	float x;
	float y = 0; // y can never be 0.
	
	scanf("%f", &x);

	if (x <= 0) {
		y = f2(x);
	} else if (x <= 2) {
		printf("no such value exists\n");
	} else if (x <= 12) {
		y = f1(x);
	} else if (x <= 22) {
		printf("no such value exists\n");
	} else if (x < 32) {
		y = f1(x);
	} else {
		printf("no such value exists\n");
	}

	if (y != 0) {
		printf("%f\n", y);
	}
	
	return 0;
}
