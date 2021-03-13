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

int main(){
	FAST_IO;

	int n, m, q;
	cin >> n >> m >> q;

	vector<pair<int, int>> bags(n);
	for(auto& [wi, vi] : bags) cin >> wi >> vi;
	vector<int> boxes(m);
	for(auto& xi : boxes) cin >> xi;

	sort(bags.begin(), bags.end(), [](auto a, auto b){
		return b.second < a.second;
	});

	int l, r;
	while(q--){
		cin >> l >> r;
		l--, r--;
		vector<int> qboxes(m - (r - l + 1));
		for(int i = 0, j = 0; i < m; i++){
			if(i < l || i > r) {
				qboxes[j] = boxes[i];
				j++;
			}
		}
		sort(qboxes.begin(), qboxes.end());
		set<int> used_boxes;
		intfl value = 0;
		for(auto bag : bags){
			for(intfl i = 0; (unsigned) i < qboxes.size(); i++){
				if(bag.first <= qboxes[i] && !used_boxes.count(i)){
					used_boxes.insert(i);
					value += bag.second;
					break;
				}
			}
		}
		cout << value << "\n";
	}

	return 0;
}
