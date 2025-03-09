#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// Amount of indices in the antidiagonal to repeat, as a coefficient
const float REPEATING_CHANCE = 0.5;

// The first antidiagonal value determines the others.
// It is MIN_FIRST + random_float(MIN_RANDOM, MAX_RANDOM)
// All the following are between MIN and MAX, but guaranteed to be less or
// equal.

/*// These are good for testing no suitable elements.*/
/*const float MIN_FIRST  = 5;*/
/*const float MIN_RANDOM = -9.99;*/
/*const float MAX_RANDOM = 10;*/

/*// The first suitable element is probably in the middle*/
/*const float MIN_FIRST  = 5;*/
/*const float MIN_RANDOM = 0;*/
/*const float MAX_RANDOM = 20;*/

/*// Good chance that none will be suitable, but some towards the end can be*/
/*const float MIN_FIRST  = 49;*/
/*const float MIN_RANDOM = -5;*/
/*const float MAX_RANDOM = 50;*/

// First 1-3 are suitable
const float MIN_FIRST = 10;
const float MIN_RANDOM = -10;
const float MAX_RANDOM = -5;

float random_float(float a, float b) {
  // If the first part is 0, we get a;
  // If 1, we get b - a + a = b;
  return ((float)rand() / (float)RAND_MAX) * (b - a) + a;
}

char contains(int array[], int size, int needle) {
  char result = 0;
  for (int i = 0; i < size; i++) {
    if (array[i] == needle) {
      result = 1;
      break;
    }
  }

  return result;
}

// Searches for an element that satisfies `(x >= 0) && (x <= 5)`
int asc_binary_search(int size, float array[size][size]) {
  int left_x_bound = 0;
  int right_x_bound = size - 1;
  float current_element;
  int final_x = -1;

  while (left_x_bound <= right_x_bound) {
    int x = (left_x_bound + right_x_bound) / 2;
    current_element = array[size - x - 1][x];
    printf("Checking %5.2f (%d %d): ", current_element, size - x - 1, x);

    if ((current_element >= 0) && (current_element <= 5)) {
      final_x = x;
      printf("found\n");
      break;
    } else if (current_element > 5) {
      right_x_bound = x - 1;
      printf("more than needed\n");
    } else {
      left_x_bound = x + 1;
      printf("less than needed\n");
    }
  }

  return final_x;
}

void generate_data(int size, float matrix[size][size]) {
  // Amount of indices in the antidiagonal to repeat
  const int REPEATING_SIZE = REPEATING_CHANCE * size;

  // Randomly generating indices that will have repeated values
  // (not guaranteed to be unique, but I'm fine with that)
  int repeating_indices[REPEATING_SIZE];
  for (int i = 0; i < REPEATING_SIZE; i++) {
    repeating_indices[i] =
        1 + random_float(0, size - 2); // we only need the whole part
  }

  float antidiagonal[size];
  // Randomly generating the antidiagonal.
  // The largest number goes first (top-to-bottom), and some of them have to
  // repeat
  antidiagonal[0] = MIN_FIRST + random_float(MIN_RANDOM, MAX_RANDOM);
  for (int i = 1; i < size; i++) {
    if (contains(repeating_indices, REPEATING_SIZE, i)) {
      antidiagonal[i] = antidiagonal[i - 1];
    } else {
      do {
        antidiagonal[i] = random_float(MIN_RANDOM, MAX_RANDOM);
      } while (antidiagonal[i] >= antidiagonal[i - 1]);
    }
  }

  // Filling the matrix with antidiagonal and arbitrary random values
  for (int y = 0; y < size; y++) {
    for (int x = 0; x < size; x++) {
      if (x == size - y - 1) {
        matrix[y][x] = antidiagonal[y];
        if ((antidiagonal[y] >= 0) && (antidiagonal[y] <= 5)) {
          printf("\033[34m"); // print in blue
        } else {
          printf("\033[31m"); // print in red
        }
      } else {
        matrix[y][x] = ((float)rand() / (float)RAND_MAX) * 99.0;
      }
      printf("%5.2f\033[0m ", matrix[y][x]);
    }
    printf("\n");
  }
}

int main(int argc, char **argv) {
  srand(clock());

  int size;
  if (argc == 1) {
    printf("Enter the matrix size (n*n): ");
    scanf("%d", &size);
  } else {
    size = atoi(argv[1]);
  }

  for (;;) {
    system("clear");

    float matrix[size][size];

    generate_data(size, matrix);

    int x = asc_binary_search(size, matrix);

    if (x == -1) {
      printf("No element could be found\n");
    } else {
      printf("The first found element is (%d %d)\n", size - x - 1, x);
    }

    printf("\x1b[2m\n\nPress Enter to regenerate...\n\x1b[0m");
    getchar();
  }

  return 0;
}
