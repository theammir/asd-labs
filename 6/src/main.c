#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

const int SHELL_GAPS[3] = {7, 3, 1};

void swap(int *a, int *b) {
  int temp = *a;
  *a = *b;
  *b = temp;
}

// Sorts the antidiagonal of a square matrix, ascension order (top-to-bottom)
void shell_sort(int size, int **array) {
  for (int g = 0; g < sizeof(SHELL_GAPS) / sizeof(int); g++) {
    int gap = SHELL_GAPS[g];
    for (int i = gap; i < size;
         i++) { // if gap is greater than size, we skip it
      for (int j = i - gap; j >= 0; j -= gap) {
        int *diagonal_j = &array[j][size - j - 1];
        int *diagonal_j_next = &array[j + gap][size - (j + gap) - 1];

        if (*diagonal_j > *diagonal_j_next) { // Condition that defines order
          swap(diagonal_j, diagonal_j_next);
        }
      }
    }
  }
}

void print_matrix(int size, int **array) {
  for (int y = 0; y < size; y++) {
    for (int x = 0; x < size; x++) {
      if (x == size - y - 1) {
        printf("\033[31m%3d\033[0m ", array[y][x]); // print in red
      } else {
        printf("%3d ", array[y][x]);
      }
    }
    printf("\n");
  }
}

int main(int argc, char **argv) {
  srand(clock());

  int size;
  int **matrix;

  if (argc < 3) {
    if (argc == 1) {
      printf("Enter the size of the matrix: ");
      scanf("%d", &size);
    } else {
      size = atoi(argv[1]);
    }

    matrix = malloc(sizeof(int *) * size);
    for (int y = 0; y < size; y++) {
      matrix[y] = malloc(size * sizeof(int));
      for (int x = 0; x < size; x++) {
        matrix[y][x] = rand() % 100 - 50;
      }
    }
  } else {
    size = atoi(argv[1]);
    if (argc < size * size + 2) {
      printf("Invalid argument count for matrix size %d\n", size);
      return 0;
    }

    matrix = malloc(sizeof(int *) * size);
    for (int i = 0; i < size * size; i++) {
      int y = i / size;
      int x = i % size;
      if (x == 0) {
        matrix[y] = malloc(size * sizeof(int));
      }
      matrix[y][x] = atoi(argv[i + 2]);
    }
  }

  printf("\nInput matrix:\n");
  print_matrix(size, matrix);

  shell_sort(size, matrix);

  printf("\nSorted matrix:\n");
  print_matrix(size, matrix);

  free(matrix);
  return 0;
}
