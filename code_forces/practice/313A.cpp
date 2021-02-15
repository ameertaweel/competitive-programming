/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out
*/
#include <bits/stdc++.h>

using namespace std;

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	int n;
	cin >> n;

	if(n >= 0){
		cout << n;
		return 0;
	}

	int lastDigit = -n % 10;
	int noLastDigit = n / 10;
	int noSecondLastDigit = (n / 100) * 10 - lastDigit;

	cout << max({noLastDigit, noSecondLastDigit});

	return 0;
}
