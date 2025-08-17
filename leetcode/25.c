// LeetCode/25 - Reverse Nodes in k-Group

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode* reverseKGroup(struct ListNode* head, int k) {
    if (k == 1) return head;

    struct ListNode* head_alt = malloc(sizeof(struct ListNode));
    head_alt -> next = head;

    struct ListNode* a = head_alt;
    struct ListNode* b = head_alt -> next -> next;
    int diff = 2;

    while (b != NULL) {
        if (diff == k) {
            struct ListNode* frst = a -> next;
            struct ListNode* prev = a -> next;
            struct ListNode* curr = a -> next -> next;
            struct ListNode* last = b -> next;
            while (curr != last) {
                struct ListNode* next = curr -> next;
                curr -> next = prev;
                prev = curr;
                curr = next;
            }
            (a -> next) -> next = last;
            a -> next = b;
            a = frst;
            if (frst -> next == NULL) break;
            b = frst -> next -> next;
            diff = 2;
        } else {
            b = b -> next;
            diff++;
        }
    }

    return head_alt -> next;
}
