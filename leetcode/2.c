/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

long long linkedListToInt(struct ListNode* head);

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    int carry = 0;

    struct ListNode* head = NULL;
    struct ListNode* curr = NULL;

    while (l1 != NULL || l2 != NULL) {
        if (head == NULL) {
            head = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = head;
        } else {
            curr->next = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = curr->next;
        }

        int op1 = 0;
        if (l1 != NULL) {
            op1 = l1->val;
            l1 = l1->next;
        }
        int op2 = 0;
        if (l2 != NULL) {
            op2 = l2->val;
            l2 = l2->next;
        }

        int sum = op1 + op2 + carry;
        int digit = sum % 10;
        carry = sum / 10;

        *curr = (struct ListNode) { digit, NULL };
    }

    if (carry > 0) {
        curr->next = (struct ListNode*) malloc(sizeof(struct ListNode));
        curr = curr->next;
        *curr = (struct ListNode) { carry, NULL };
    }

    return head;
}
