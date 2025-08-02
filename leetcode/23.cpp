// LeetCode/23 - Merge k Sorted Lists

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
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        return mergeKListsRange(lists, 0, lists.size());
    }

    ListNode* mergeKListsRange(vector<ListNode*>& lists, size_t a, size_t b) {
        if (b <= a) return NULL;
        if (b - a == 1) return lists[a];
        if (b - a == 2) return merge2Lists(lists[a], lists[a + 1]);;

        size_t mid = a + (b - a) / 2;

        return merge2Lists(
            mergeKListsRange(lists, a, mid),
            mergeKListsRange(lists, mid, b)
        );
    }

    ListNode* mergeKListsAlternative2(vector<ListNode*>& lists) {
        ListNode* r = NULL;

        for (size_t i = 0; i < lists.size(); i++) {
            r = merge2Lists(r, lists[i]);
        }

        return r;
    }

    ListNode* merge2Lists(ListNode* a, ListNode* b) {
        if (a == NULL) return b;
        if (b == NULL) return a;

        ListNode* r = NULL;
        if ((a -> val) < (b -> val)) {
            r = a;
            a = a -> next;
        } else {
            r = b;
            b = b -> next;
        }
        r -> next = NULL;
        auto c = r;

        while (a != NULL && b != NULL) {
            if ((a -> val) < (b -> val)) {
                c -> next = a;
                a = a -> next;
            } else {
                c -> next = b;
                b = b -> next;
            }
            c -> next -> next = NULL;
            c = c -> next;
        }

        if (a == NULL) {
            c -> next = b;
        } 
        if (b == NULL) {
            c -> next = a;
        }

        return r;
    }

    ListNode* mergeKListsAlternative1(vector<ListNode*>& lists) {
        std::priority_queue<std::pair<int, size_t>> heap;

        for (size_t i = 0; i < lists.size(); i++) {
            if (lists[i] != NULL) {
                heap.push({-1 * (lists[i] -> val), i});
                lists[i] = lists[i] -> next;
            }
        }

        if (heap.size() == 0) {
            return NULL;
        }

        auto ans = new ListNode();
        auto cur = ans;

        while (heap.size() > 0) {
            auto top = heap.top();
            heap.pop();
            auto v = -1 * top.first;
            auto i = top.second;
            cur -> val = v;
            if (lists[i] != NULL) {
                heap.push({-1 * (lists[i] -> val), i});
                lists[i] = lists[i] -> next;
            }
            if (heap.size() > 0) {
                cur -> next = new ListNode();
                cur = cur -> next;
            }
        }

        return ans;
    }
};
