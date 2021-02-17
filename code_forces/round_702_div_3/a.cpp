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

		int steps = 0;
		int prev_element;
		int current_element;
		cin >> prev_element;
		n--;
		while(n--){
			cin >> current_element;
			int max_element = max(prev_element, current_element);
			int min_element = min(prev_element, current_element);

			if((double) max_element / min_element > 2){
				while(min_element * 2 < max_element) steps++, min_element *= 2;
			}

			prev_element = current_element;
		}

		cout << steps << "\n";
	}

	return 0;
}
