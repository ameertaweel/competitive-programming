struct DListNode {
    int key;
    int val;
    struct DListNode* prev;
    struct DListNode* next;
};

class LRUCache {
private:
    int capacity;
    int len;
    std::unordered_map<int, struct DListNode*> map;
    struct DListNode* head;
    struct DListNode* tail;

    void insert(DListNode* node) {
        if (len == 0) {
            head = node;
            tail = node;
            node->prev = NULL;
            node->next = NULL;
        } else {
            head->prev = node;
            node->prev = NULL;
            node->next = head;
            head = node;
        }
        len++;
    }

    void remove(DListNode* node) {
        auto prev = node->prev;
        auto next = node->next;

        if (node == head) head = next;
        else prev->next = next;

        if (node == tail) tail = prev;
        else next->prev = prev;

        len--;
    }

public:
    LRUCache(int capacity) {
        this->capacity = capacity;
        this->len = 0;
        this->head = NULL;
        this->tail = NULL;
    }
    
    int get(int key) {
        if (!map.contains(key)) return -1;
        auto node = map[key];
        remove(node);
        insert(node);
        return node->val;
    }
    
    void put(int key, int value) {
        if (map.contains(key)){
            auto node = map[key];
            node->val = value;
            remove(node);
            insert(node);
            return;
        }

        if (this->len == this->capacity) {
            auto tail_node = tail;
            map.erase(tail_node->key);
            remove(tail_node);
            free(tail_node);
        }

        auto new_node = (struct DListNode*) malloc(sizeof(struct DListNode));
        *new_node = { key, value, NULL, NULL };
        insert(new_node);
        map.insert({ key, new_node });
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */
