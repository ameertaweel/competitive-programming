/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode* next;
 * };
 */
struct ListNode* removeNthFromEnd(struct ListNode* head, int n) {
    int len = 0;
    struct ListNode* curr = head;
    while (curr != NULL) {
        len++;
        curr=curr->next;
    }
    if (len == n) return head->next;
    curr = head;
    while (len > n + 1) {
        len--;
        curr=curr->next;
    }
    curr->next = curr->next->next;
    return head;
}
