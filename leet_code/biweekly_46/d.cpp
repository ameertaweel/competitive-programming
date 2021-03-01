/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out && ./out < in
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t
#define ll long long

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

		vector<bool> visited(n);
		deque<int> d;
		d.push_back(0);
		visited[0] = true;

		vector<int> last_visited_children(n, 0);
		unordered_map<int, stack<int>> parents_values;
		vector<int> parents_values_indexes(n, 0);
		unordered_map<string, int> gcds;
		vector<int> result(n, -1);

		int next_parent_index = -1;
		while(!d.empty()){
			auto& node_index = d.back();
			auto& node = g[node_index];
			parents_values[nums[node_index]].push(node_index);
			next_parent_index++;
			parents_values_indexes[node_index] = next_parent_index;

			bool found_unvisited_child = false;
			int last_visited_child_index = last_visited_children[node_index];
			for(; last_visited_child_index < node.size(); last_visited_child_index++){
				int child = node[last_visited_child_index];
				if(visited[child]) continue;
				auto& child_val = nums[child];
				for(auto& [n, s] : parents_values){
					if(s.empty()) continue;
					string key = to_string(child_val) + "," + to_string(n);
					if(!gcds.count(key)){
						string reverse_key = to_string(n) + "," + to_string(child_val);
						int gcd = __gcd(child_val, n);
						gcds[key] = gcd;
						gcds[reverse_key] = gcd;
					}
					if(gcds[key] == 1 && (result[child] == -1 || parents_values_indexes[s.top()] > parents_values_indexes[result[child]])) result[child] = s.top();
				}
				d.push_back(child);
				visited[child] = true;
				found_unvisited_child = true;
				break;
			}
			last_visited_children[node_index] = last_visited_child_index;
			if(!found_unvisited_child) {
				d.pop_back();
				parents_values[nums[node_index]].pop();
				next_parent_index--;
			}
		}

		return result;
    }
};
