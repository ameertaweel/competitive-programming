#include <iostream>

using namespace std;

int main(){
	// Read data
	int a, b, c;
	cin >> a >> b >> c;

	// The one who starts needs at least an extra cookie to win
	if(c) cout << (b > a ? "Aoki" : "Takahashi");
	else cout << (a > b ? "Takahashi" : "Aoki");

	return 0;
}
