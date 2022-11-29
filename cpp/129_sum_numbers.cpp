#include "TreeNode.h"
#include <catch2/catch_test_macros.hpp>

int recursion(TreeNode* root, int p)
{
  if (!root)
    return 0;

  // UNSCOPED_INFO("the value is " << root->val);
  int sum{p * 10 + root->val};

  if (!root->left && !root->right)
    return sum;
  else
    return recursion(root->left, sum) + recursion(root->right, sum);
}

int sumNumbers(TreeNode* root)
{
  return recursion(root, 0);
}

TEST_CASE("lc129")
{
  TreeNode a1l{2};
  TreeNode a1r{3};
  TreeNode a1(1, &a1l, &a1r);

  REQUIRE(sumNumbers(&a1) == 25);
}
