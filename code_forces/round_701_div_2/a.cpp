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
		long a, b;
		cin >> a >> b;

		long current = 0;
		if(b == 1){
			b++, current++;
		}

		long min_op = 1e9;
		while(true){
			long tmp_a = a;
			long op = current;
			while(tmp_a){
				tmp_a /= b;
				op++;
			}
			if(op <= min_op) min_op = op;
			else break;
			b++, current++;
		}

		cout << min_op << "\n";
	}

	return 0;
}
