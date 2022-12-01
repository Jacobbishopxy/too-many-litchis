#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

bool searchMatrix(const vector<vector<int>>& matrix, int target)
{
  int m{(int)matrix.size()}, n{(int)matrix[0].size()};

  int top{0}, bottom{m * n - 1};
  while (top <= bottom)
  {
    int mid{(bottom - top) / 2 + top};

    // 拉平 m * n 矩阵为头尾相连的数组，mid % n 为原始列位置
    int x{matrix[mid / n][mid % n]};
    if (x == target)
      return true;
    if (x < target)
      top = mid + 1;
    else
      bottom = mid - 1;
  }

  return false;
}

TEST_CASE("lc74")
{
  REQUIRE(searchMatrix({{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}}, 3) == true);
  REQUIRE(searchMatrix({{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}}, 13) == false);
}
