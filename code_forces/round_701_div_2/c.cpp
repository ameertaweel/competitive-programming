/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

#define endl "\n";

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	int t;
	cin >> t;

	while(t--){
		int64_t x, y;
		cin >> x >> y;

		int64_t counter = 0;
		int64_t n = 1;
		for(;;){ // Forever
			int64_t start = n + 1;
			int64_t end = (x - n) / n;
			end = end > y ? y : end;
			if(start > x || start > end) break;
			counter += end - start + 1;
			n++;
		}
		cout << counter << endl;
	}

	return 0;
}
