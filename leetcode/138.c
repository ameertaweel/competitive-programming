/**
 * Definition for a Node.
 * struct Node {
 *     int val;
 *     struct Node* next;
 *     struct Node* random;
 * };
 */
struct Node* copyRandomList(struct Node* head) {
    struct Node* curr = head;

    struct Node* copy_head = NULL;
    struct Node* copy_curr = NULL;

    while (curr != NULL) {
        copy_curr = (struct Node*) malloc(sizeof(struct Node));

        if (copy_head == NULL) copy_head = copy_curr;

        *copy_curr = (struct Node) { curr->val, curr->next, NULL };
        curr->next = copy_curr;
        curr = copy_curr->next;
    }

    curr = head;
    while (curr != NULL) {
        struct Node* copy_curr = curr->next;
        struct Node* copy_rand = curr->random == NULL ? NULL : curr->random->next;
        copy_curr->random = copy_rand;
        curr = copy_curr->next;
    }

    curr = head;
    while (curr != NULL) {
        struct Node* copy_curr = curr->next;
        struct Node* temp = copy_curr->next;
        copy_curr->next = copy_curr->next == NULL ? NULL : copy_curr->next->next;
        curr->next = temp;
        curr = temp;
    }

    return copy_head;
}
