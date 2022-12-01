#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

int findPeakElement(const vector<int>& nums)
{
  int left{0}, right{(int)nums.size() - 1};

  while (left < right)
  {
    int mid{(left + right) / 2};

    if (nums[mid] < nums[mid + 1])
      left = mid + 1;
    else
      right = mid;
  }

  return left;
}

TEST_CASE("lc162")
{
  REQUIRE(findPeakElement({1, 2, 3, 1}) == 2);
  REQUIRE(findPeakElement({1, 2, 1, 3, 5, 6, 4}) == 5);
}
