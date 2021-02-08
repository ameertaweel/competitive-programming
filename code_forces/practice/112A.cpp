#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	string a, b;
	cin >> a >> b;

	transform(a.begin(), a.end(), a.begin(), ::toupper);
	transform(b.begin(), b.end(), b.begin(), ::toupper);

	if(a == b) cout << 0;
	else cout << (a > b ? 1 : -1);

	return 0;
}
