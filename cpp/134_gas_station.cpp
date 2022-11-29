#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

int canCompleteCircuit(const vector<int>& gas, const vector<int>& cost)
{
  const int n{static_cast<int>(gas.size())};
  int i{0};

  while (i < n)
  {
    int sumOfGas{0}, sumOfCost{0};
    int cap{0};

    while (cap < n)
    {
      int j = (i + cap) % n;
      sumOfGas += gas[j];
      sumOfCost += cost[j];

      if (sumOfGas < sumOfCost)
        break;

      ++cap;
    }

    if (cap == n)
      return i;
    else
      i = i + cap + 1;
  }

  return -1;
}

TEST_CASE("lc134")
{
  REQUIRE(canCompleteCircuit({1, 2, 3, 4, 5}, {3, 4, 5, 1, 2}) == 3);

  REQUIRE(canCompleteCircuit({2, 3, 4}, {3, 4, 3}) == -1);
}
