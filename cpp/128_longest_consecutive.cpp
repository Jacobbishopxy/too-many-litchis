#include <algorithm>
#include <catch2/catch_test_macros.hpp>
#include <set>
#include <vector>

using namespace std;

int longestConsecutive(const vector<int>& nums)
{
    set<int> s(nums.begin(), nums.end());
    int longest_streak { 0 };

    for (const int& i : nums) {
        if (!s.count(i - 1)) {
            int current_num = i;
            int current_streak = 1;

            while (s.count(current_num + 1)) {
                ++current_num;
                ++current_streak;
            }

            longest_streak = max(longest_streak, current_streak);
        }
    }
    return longest_streak;
}

TEST_CASE("lc128")
{
    REQUIRE(longestConsecutive(vector<int> { 100, 4, 200, 1, 3, 2 }) == 4);
    REQUIRE(longestConsecutive(vector<int> { 0, 3, 7, 2, 5, 8, 4, 6, 0, 1 }) == 9);
}
