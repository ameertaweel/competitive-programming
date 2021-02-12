/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	long n, q, k;
	cin >> n >> q >> k;

	vector<long> a(n);
	for(auto& ai : a){
		cin >> ai;
	}

	while(q--){
		long l, r;
		cin >> l;
		cin >> r;
		l--, r--;

		long permutations = 0;
		/* for(long i = l; i < r + 1; i++){ */
		/* 	long prev = i != l ? a[i - 1] : 0; */
		/* 	long next = i != r ? a[i + 1] : k + 1; */
		/* 	permutations += next - prev - 2; */
		/* } */
		long first = a[l] - 1;
		long last = k - a[r];
		long size = r - l + 1;
		long remaining = k - first - last - size;
		permutations = first + 2 * remaining + last;

		cout << permutations << "\n";
	}

	return 0;
}
