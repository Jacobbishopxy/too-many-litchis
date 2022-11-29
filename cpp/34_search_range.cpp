#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

int binarySearch(const vector<int>& nums, int target, bool lower)
{
  int left{0}, right{(int)nums.size() - 1}, ans{(int)nums.size()};
  while (left <= right)
  {
    int mid{(left + right) / 2};
    if (nums[mid] > target || (lower && nums[mid] >= target))
    {
      right = mid - 1;
      ans = mid;
    }
    else
      left = mid + 1;
  }

  return ans;
}

vector<int> searchRange(const vector<int>& nums, int target)
{
  int leftIdx{binarySearch(nums, target, true)};
  int rightIdx{binarySearch(nums, target, false) - 1};
  if (leftIdx <= rightIdx &&
      rightIdx < nums.size() &&
      nums[leftIdx] == target &&
      nums[rightIdx] == target)
    return vector<int>{leftIdx, rightIdx};

  return vector<int>{-1, -1};
}

TEST_CASE("lc34")
{
  REQUIRE(searchRange({5, 7, 7, 8, 8, 10}, 8) == vector<int>{3, 4});
}
