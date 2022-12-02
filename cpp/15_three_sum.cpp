#include <algorithm>
#include <catch2/catch_test_macros.hpp>
#include <set>
#include <vector>

using namespace std;

vector<vector<int>> threeSum(vector<int>& nums)
{
  // 排序
  sort(nums.begin(), nums.end());

  int n{(int)nums.size()};
  vector<vector<int>> res{};

  for (int first{0}; first < n; ++first)
  {
    // 跳过相同的元素
    if (first > 0 && nums[first] == nums[first - 1])
      continue;

    int third{n - 1}; // 数组最右端
    int target{-nums[first]};

    // 第二个元素
    for (int second{first + 1}; second < n; ++second)
    {
      // 跳过相同的元素
      if (second > first + 1 && nums[second] == nums[second - 1])
        continue;

      // 确保第二个指针在第三个指针左侧，大于目标则继续往右移动指针
      while (second < third && nums[second] + nums[third] > target)
        --third;

      // 指针相同时退出循环
      if (second == third)
        break;

      // 符合条件，记录
      if (nums[second] + nums[third] == target)
        res.push_back({nums[first], nums[second], nums[third]});
    }
  }

  return res;
}

TEST_CASE("lc15")
{
  vector<int> v{-1, 0, 1, 2, -1, -4};
  REQUIRE(threeSum(v) == vector<vector<int>>{{-1, -1, 2}, {-1, 0, 1}});
}
