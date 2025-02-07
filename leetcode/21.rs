/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode* next;
 * };
 */
struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
    struct ListNode* head = NULL;
    struct ListNode* curr = NULL;

    while (list1 != NULL && list2 != NULL) {
        if (head == NULL) {
            head = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = head;
        } else {
            curr->next = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = curr->next;
        }
        *curr = (struct ListNode) { 0, NULL };
        if (list1->val < list2->val) {
            curr->val = list1->val;
            list1 = list1->next;
        } else {
            curr->val = list2->val;
            list2 = list2->next;
        }
    }

    while (list1 != NULL) {
        if (head == NULL) {
            head = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = head;
        } else {
            curr->next = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = curr->next;
        }
        *curr = (struct ListNode) { 0, NULL };
        curr->val = list1->val;
        list1 = list1->next;
    }

    while (list2 != NULL) {
        if (head == NULL) {
            head = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = head;
        } else {
            curr->next = (struct ListNode*) malloc(sizeof(struct ListNode));
            curr = curr->next;
        }
        *curr = (struct ListNode) { 0, NULL };
        curr->val = list2->val;
        list2 = list2->next;
    }

    return head;
}
