/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    // Simple addition in columns algo implemented with recursion.
    ListNode* addTwoNumbers(ListNode *l1, ListNode *l2, int overflow = 0) {
        if (l1 == NULL && l2 == NULL) {
            if (overflow == 1) return new ListNode(1);
            else return nullptr;
        }

        int sum = 0;
        ListNode *l1_next = NULL;
        ListNode *l2_next = NULL;
        if (l1 != nullptr) {
            sum += l1->val;
            l1_next = l1->next;
        }
        if (l2 != nullptr) {
            sum += l2->val;
            l2_next = l2->next;
        }
        sum += overflow;

        ListNode* r = new ListNode(sum % 10);
        r->next = addTwoNumbers(l1_next, l2_next, sum / 10);
        return r;
    }
};
