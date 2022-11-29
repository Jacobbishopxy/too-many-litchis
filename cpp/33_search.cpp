#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

int search1(const vector<int>& nums, int target)
{
  int n = (int)nums.size();
  if (!n)
    return -1;
  if (n == 1)
    return nums[0] == target ? 0 : -1;

  int left{0}, right{n - 1};
  while (left <= right)
  {
    int mid{(left + right) / 2}; // 二分定位中间
    if (nums[mid] == target)
      return mid;

    if (nums[0] <= nums[mid]) // 左边部分为有序序列
    {
      if (nums[0] <= target && target < nums[mid]) // 验证目标是否位于左边部分
        right = mid - 1;                           // 位于左边部分，缩小右边界
      else
        left = mid + 1; // 位于右边部分，直接移动左边界至中间
    }
    else // 右边部分为有序数列
    {
      if (nums[mid] < target && target <= nums[n - 1]) // 验证目标是否位于右边部分
        left = mid + 1;                                // 位于右边部分，缩小左边界
      else
        right = mid - 1; // 位于左边部分，直接移动右边界至中间
    }
  }

  return -1;
}

int search2(const vector<int>& nums, int target)
{
  int left{0}, right{(int)nums.size() - 1};
  while (left < right)
  {
    int mid{(left + right) / 2}; // 二分定位中间
    if (nums[mid] == target)
      return mid;

    if (nums[left] <= nums[mid]) // 左边部分为有序序列
    {
      if (nums[left] <= target && target < nums[mid]) // 验证目标是否位于左边部分
        right = mid - 1;                              // 位于左边部分，缩小右边界
      else
        left = mid + 1; // 位于右边部分，直接移动左边界至中间
    }
    else // 右边部分为有序数列
    {
      if (nums[mid] < target && target <= nums[right]) // 验证目标是否位于右边部分
        left = mid + 1;                                // 位于右边部分，缩小左边界
      else
        right = mid - 1; // 位于左边部分，直接移动右边界至中间
    }
  }

  return nums[left] == target ? left : -1;
}

TEST_CASE("lc33")
{
  REQUIRE(search1({4, 5, 6, 7, 0, 1, 2}, 0) == 4);

  REQUIRE(search1({3, 1}, 1) == 1);

  REQUIRE(search1({5, 1, 3}, 5) == 0);

  REQUIRE(search2({4, 5, 6, 7, 0, 1, 2}, 0) == 4);

  REQUIRE(search2({3, 1}, 1) == 1);

  REQUIRE(search2({5, 1, 3}, 5) == 0);
}
