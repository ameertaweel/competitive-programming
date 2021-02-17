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

	intfl t;
	cin >> t;

	while(t--){
		intfl n;
		cin >> n;

		map<intfl, intfl> counts;
		for(intfl i = 0; i < n; i++){
			intfl num;
			cin >> num;
			counts[num]++;
		}


		map<intfl, intfl> count_groups;
		for(auto [num, count] : counts){
			count_groups[count]++;
		}

		intfl counts_count = counts.size();
		intfl left = 0;
		intfl right = n;
		intfl min_moves = n;
		for(auto [count, reps] : count_groups){
			intfl moves = 0;
			moves += left;
			counts_count -= reps;
			left += count * reps;
			right -= count * reps;
			moves += right - counts_count * count;

			if(moves < min_moves) min_moves = moves;
		}
		cout << min_moves << "\n";
	}

	return 0;
}
