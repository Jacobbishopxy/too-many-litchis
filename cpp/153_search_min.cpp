#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

int findMin(const vector<int>& nums)
{
  int left{0}, right{(int)nums.size() - 1};

  while (left < right)
  {
    int mid{(left + right) / 2};

    if (nums[mid] < nums[right])
      right = mid; // 忽略右侧部分
    else
      left = mid + 1; // 忽略左侧部分
  }

  return nums[left];
}

TEST_CASE("lc153")
{
  REQUIRE(findMin({4, 5, 6, 7, 0, 1, 2}) == 0);
}
