#include "ListNode.h"
#include <catch2/catch_test_macros.hpp>
#include <vector>

using namespace std;

/**
 * 该方法并没有释放被删除的链表节点以及哑节点
 */
ListNode* deleteDuplicates(ListNode* head)
{
  if (!head)
    return head;

  // 头节点可能会被删除，需要一个哑节点指向链表的头节点
  ListNode* dummy = new ListNode(0, head);

  ListNode* cur = dummy;

  while (cur->next && cur->next->next) // 下一节点与下下节点均存在
  {
    if (cur->next->val == cur->next->next->val) // 如果下一节点与下下节点的值相同
    {
      int x = cur->next->val;                  // 临时记录该值
      while (cur->next && cur->next->val == x) // 下一节点的值等于 x 情况下进行循环
      {
        cur->next = cur->next->next; // 将当前指向下一节点，指向下下节点
      }
    }
    else // 值不同，则将 cur 移动到下一节点
    {
      cur = cur->next;
    }
  }

  return dummy->next;
}

vector<int> listNodeToVector(ListNode* head)
{
  vector<int> res{};

  while (head)
  {
    res.push_back(head->val);
    head = head->next;
  }

  return res;
}

TEST_CASE("lc82")
{
  ListNode* a7{new ListNode(5, nullptr)};
  ListNode* a6{new ListNode(4, a7)};
  ListNode* a5{new ListNode(4, a6)};
  ListNode* a4{new ListNode(3, a5)};
  ListNode* a3{new ListNode(3, a4)};
  ListNode* a2{new ListNode(2, a3)};
  ListNode* a1{new ListNode(1, a2)};

  REQUIRE(listNodeToVector(deleteDuplicates(a1)) == vector<int>{1, 2, 5});
}
