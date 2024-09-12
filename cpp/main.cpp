#include <cstddef>
#include <iostream>
#include <string>
using namespace std;

pair<size_t, size_t> matrix(int num, int matrix[][2], int rows, int columns) {
  pair<size_t, size_t> result =
  { -1,
    -1 }

  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < columns; j++) {
      if (matrix[i][j] == num) {
        result = {i, j};
        return result;
      };
    };
  };
  return result;
};

int main() {
  int matrix1[2][2] = {{2, 3}, {1, 5}};
  auto result = matrix(3, matrix1, 2, 2);

  if (result.first != (size_t)-1) {
    cout << "The number is at i = " << result.first << ", j = " << result.second
         << " in the matrix." << endl;
  } else {
    cout << "Number not found in the matrix." << endl;
  }

  return 0;
}
