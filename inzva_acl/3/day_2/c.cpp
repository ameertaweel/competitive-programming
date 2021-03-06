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

int main(){
	FAST_IO;

	intfl n;
	cin >> n;

	vector<pair<bool, vector<intfl>>> graph(n);
	for(intfl i = 0; i < n; i++){
		intfl a, b;
		cin >> a >> b;
		graph[a - 1].second.push_back(b - 1);
		graph[b - 1].second.push_back(a - 1);
	}

	vector<intfl> rows;

	queue<pair<intfl, intfl>> q;
	// 0 is the root of the tree
	q.push({0, 0});
	graph[0].first = true;

	while(!q.empty()){
		auto& [index, depth] = q.front();
		q.pop();
		auto& node = graph[index];
		if((unsigned) rows.size() == depth) rows.push_back(0);
		rows[depth]++;
		for(auto& connection_index : node.second){
			auto& connection = graph[connection_index];
			if(!connection.first){
				q.push({connection_index, depth + 1});
				connection.first = true;
			}
		}
	}

	for(auto& row : rows){
		cout << row << endl;
	}
	return 0;
}
