/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out && ./out < in
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t

#define FAST_IO \
ios::sync_with_stdio(0);\
cin.tie(0);\
cout.tie(0);

class Solution {
public:
    vector<int> getCoprimes(vector<int>& nums, vector<vector<int>>& edges) {
		int n = nums.size();
		vector<vector<int>> g(n, vector<int>{});

		for(auto& edge : edges){
			g[edge[0]].push_back(edge[1]);
			g[edge[1]].push_back(edge[0]);
		}

		vector<vector<int>> parents(n, vector<int>{});
		vector<bool> visited(n);
		queue<int> q;
		q.push(0);
		visited[0] = true;

		while(!q.empty()){
			auto& node_children = g[q.front()];
			auto& node_parents = parents[q.front()];

			for(auto& child : node_children){
				if(visited[child]) continue;
				for(auto& parent : node_parents) parents[child].push_back(parent);
				parents[child].insert(parents[child].begin(), q.front());
				q.push(child);
				visited[child] = true;
			}

			q.pop();
		}

		int i = 0;
		vector<int> result(n, -1);
		for(auto& node_parents : parents){
			auto& node_val = nums[i];
			for(auto& parent : node_parents){
				auto& parent_val = nums[parent];
				if(__gcd(node_val, parent_val) == 1){
					result[i] = parent;
					break;
				}
			}
			i++;
		}

		return result;
    }
};
