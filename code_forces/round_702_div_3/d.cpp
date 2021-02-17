/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
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

void calc_depths(vector<int>&, vector<int>&, int = 0, int = 0, int = -1, bool = true);

int main(){
	FAST_IO;

	int t;
	cin >> t;

	while(t--){
		int n;
		cin >> n;

		vector<int> a(n);
		for(int& ai : a){
			cin >> ai;
		}

		vector<int> depths(n);
		calc_depths(a, depths);
		for(int& depth : depths){
			cout << depth << " ";
		}
		cout << "\n";
	}

	return 0;
}

void calc_depths(vector<int>& a, vector<int>& depths, int depth, int start, int end, bool init_end){
	end = init_end ? a.size() - 1 : end;

	if(start > end) return;

	int max = -1;
	int max_index = -1;
	for(int i = start; i <= end; i++){
		if(a[i] > max) max = a[i], max_index = i;
	}

	depths[max_index] = depth;

	calc_depths(a, depths, depth + 1, start, max_index - 1, false);
	calc_depths(a, depths, depth + 1, max_index + 1, end, false);
}
