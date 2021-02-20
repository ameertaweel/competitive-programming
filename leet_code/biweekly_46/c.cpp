#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t

class Solution {
public:
    vector<vector<int>> highestPeak(vector<vector<int>>& isWater) {
        int m{(int) isWater.size()}, n{(int) isWater[0].size()};

        vector<vector<int>> h(m, vector<int>(n, 0)); // Base height is 0
        vector<vector<bool>> visited(m, vector<bool>(n, false));

        queue<pair<int, int>> q;
        for(int i = 0; i < m; i++) for(int j = 0; j < n; j++){
            if(isWater[i][j]){
                q.push({i, j});
                visited[i][j] = true;
            }
        }

        while(!q.empty()){
            auto [i, j] = q.front();
            q.pop();

            if(i > 0 && !visited[i - 1][j]) {h[i - 1][j] = h[i][j] + 1; visited[i - 1][j] = true; q.push({i - 1, j});}
            if(i < m - 1 && !visited[i + 1][j]) {h[i + 1][j] = h[i][j] + 1; visited[i + 1][j] = true; q.push({i + 1, j});}
            if(j > 0 && !visited[i][j - 1]) {h[i][j - 1] = h[i][j] + 1; visited[i][j - 1] = true; q.push({i, j - 1});}
            if(j < n - 1 && !visited[i][j + 1]) {h[i][j + 1] = h[i][j] + 1; visited[i][j + 1] = true; q.push({i, j + 1});}
        }

        return h;
    }
};
