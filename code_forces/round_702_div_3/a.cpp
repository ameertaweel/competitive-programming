/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	int t;
	cin >> t;

	while(t--){
		int n;
		cin >> n;

		vector<int> a(n);

		int steps = 0;
		int prev_element;
		int current_element;
		cin >> prev_element;
		bool read_next = true;
		while(n--){
			if(read_next) cin >> current_element;
			int max_element = max(prev_element, current_element);
			int min_element = min(prev_element, current_element);

			if(max_element / min_element > 2){
				current_element = max_element / min_element + (max_element % min_element ? 1 : 0);
				steps++;
				read_next = false;
			} else {
				prev_element = current_element;
				read_next = true;
			}
		}
		cout << steps << "\n";
	}

	return 0;
}
