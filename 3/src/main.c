#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main(void) {
  srand(clock());

  int size;
  printf("Enter the matrix size (n*n): ");
  scanf("%d", &size);
  printf("\n");

  float matrix[size][size];

  for (int y = 0; y < size; y++) {
    for (int x = 0; x < size; x++) {
      matrix[y][x] = ((float)rand() / (float)RAND_MAX) * 99.0;
      /*matrix[y][x] = abs(x - y); // i'm leaving it here for testing multiple
       * occurences*/

      if (x == size - y - 1) {
        printf("\033[31m%5.2f\033[0m ", matrix[y][x]); // print in red
      } else {
        printf("%5.2f ", matrix[y][x]);
      }
    }
    printf("\n");
  }

  for (;;) { // no base case, just ctrl+c
    float needle;
    printf("\nSearch for an element in the antidiagonal: ");
    scanf("%f", &needle);

    int needle_y = size;
    // We're searching the antidiagonal. [y][size - y - 1]
    for (int y = size; y >= 0; y--) {
      if (roundf(needle * 100.0) == roundf(matrix[y][size - y - 1] * 100.0)) {
        needle_y = y;
        break;
      }
    }

    if (needle_y == size) {
      printf("No element could be found.");
    } else {
      printf("The first found element is (%d %d)", needle_y,
             size - needle_y - 1);
    }
  }

  return 0;
}
