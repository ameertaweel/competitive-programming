#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	string situation;
	cin >> situation;

	bool team_0_danger = situation.find("0000000") != string::npos;
	bool team_1_danger = situation.find("1111111") != string::npos;

	cout << (team_0_danger || team_1_danger ? "YES" : "NO");

	return 0;
}
