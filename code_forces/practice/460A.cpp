/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	int n, m;
	cin >> n >> m;

	int current_socks = n;
	int total_days = 0;
	int days_in_hand = 0;

	while(current_socks){
		total_days += current_socks;
		int passed_days = current_socks + days_in_hand;
		current_socks = passed_days / m;
		days_in_hand = passed_days % m;
	}

	cout << total_days;

	return 0;
}
