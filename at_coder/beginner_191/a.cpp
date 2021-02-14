#include <iostream>

using namespace std;

int main(){
	double v, t, s, d;
	cin >> v >> t >> s >> d;

	double battinguTime = d / v;

	bool canBattu = battinguTime < t || battinguTime > s;

	cout << (canBattu ? "Yes" : "No");

	return 0;
}
