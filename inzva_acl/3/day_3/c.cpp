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

	intfl n, l, k;
	cin >> n >> l >> k;

	vector<vector<intfl>> drivers(n - 1, vector<intfl>(l, 0));
	for(intfl i = 0; i < n - 1; i++){
		auto& driver = drivers[i];
		for(intfl j = 0; j < l; j++){
			auto& time = driver[j];
			cin >> time;
			if(j) time += driver[j - 1];
		}
	}
	intfl last_time = 0;
	for(intfl lap = 0; lap < l; lap++){
		vector<intfl> times(n - 1);
		for(intfl j = 0; j < n - 1; j++) times[j] = drivers[j][lap];
		sort(times.begin(), times.end());
		unordered_map<intfl, intfl> ranks;
		intfl current_rank = 0;
		intfl current_time = times[0];
		ranks[1] = current_time;
		for(auto& time : times){
			current_rank++;
			if(time != current_time){
				ranks[current_rank] = time;
			}
			current_time = time;
		}

		intfl this_lap_time;
		if(!ranks[k] && current_rank != k - 1){
			this_lap_time = -1;
		} else {
			if(k == 1){
				this_lap_time = 1;
			} else {
				this_lap_time = ranks[k - 1] + 1;
			}
		}
		if(this_lap_time == -1){
			cout << -1;
			break;
		}
		cout << this_lap_time - last_time << " ";
		last_time = this_lap_time;
	}

	return 0;
}
