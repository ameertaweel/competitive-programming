int trap(int* height, int n) {
    int max_val = 0;
    int max_idx = 0;

    for (int i = 0; i < n; i++) {
        if (height[i] > max_val) {
            max_val = height[i];
            max_idx = i;
        }
    }

    int ans = 0;

    int max_so_far_1 = 0;
    for (int i = 0; i < max_idx; i++) {
        if (height[i] > max_so_far_1) {
            max_so_far_1 = height[i];
        }
        ans += max_so_far_1 - height[i];
    }

    int max_so_far_2 = 0;
    for (int i = n - 1; i > max_idx; i--) {
        if (height[i] > max_so_far_2) {
            max_so_far_2 = height[i];
        }
        ans += max_so_far_2 - height[i];
    }

    return ans;
}

