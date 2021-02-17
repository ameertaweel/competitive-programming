/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	long t;
	cin >> t;

	while(t--){
		long n;
		cin >> n;

		long balance = n / 3;

		long c0, c1, c2;
		c0 = c1 = c2 = 0;
		long temp_var;
		while(n--){
			cin >> temp_var;
			switch(temp_var % 3){
				case 0: c0++; break;
				case 1: c1++; break;
				default: c2++;
			}
		}

		long steps = 0;

		while(c0 != balance || c1 != balance || c2 != balance){
			if(c0 > balance){
				long diff = c0 - balance;
				c1 += diff;
				steps += diff;
				c0 = balance;
			}
			if(c1 > balance){
				long diff = c1 - balance;
				c2 += diff;
				steps += diff;
				c1 = balance;
			}
			if(c2 > balance){
				long diff = c2 - balance;
				c0 += diff;
				steps += diff;
				c2 = balance;
			}
		}

		cout << steps << "\n";
	}

	return 0;
}
