/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode* next;
 * };
 */

struct ListNode* reverseList(struct ListNode* head);

void reorderList(struct ListNode* head) {
    struct ListNode* fast = head;
    struct ListNode* slow = head;
    struct ListNode* midd = NULL;

    while (fast != NULL && fast->next != NULL) {
        fast = fast->next->next;
        midd = slow;
        slow = slow->next;
    }

    midd = fast != NULL ? slow : midd;

    struct ListNode* half1 = head;
    struct ListNode* half2 = midd->next;

    midd->next = NULL;
    half2 = reverseList(half2);

    struct ListNode* curr = half1;
    int i = 0;

    while (curr != NULL) {
        if (i % 2 == 0) {
            half1 = half1->next;
            curr->next = half2;
            curr = half2;
        } else {
            half2 = half2->next;
            curr->next = half1;
            curr = half1;
        }
        i++;
    }
}

struct ListNode* reverseList(struct ListNode* head) {
    struct ListNode* prev = NULL;
    struct ListNode* curr = head;
    struct ListNode* next = NULL;

    while (curr != NULL) {
        next = curr->next;
        curr->next = prev;
        prev = curr;
        curr = next;
    }

    return prev;
}
