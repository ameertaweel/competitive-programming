#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	long n, m;
	cin >> n >> m;

	long points = n * m;
	bool turn = true;
	while(points > 0){
		n--, m--;
		points = n * m;
		turn = !turn;
	}

	cout << (turn ? "Malvika" : "Akshat");

	// This can be solve in constant time as well
	// The solution depends on the parity of min(n, m)

	return 0;
}
