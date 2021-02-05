#include <iostream>

using namespace std;

int main(){
	// Read the three numbers a, b, c from standard input.
	int a, b, c;
	cin >> a >> b >> c;

	// Read the string s from standard input.
	string s;
	cin >> s;

	// Calculate the sum of a, b, and c then output it along with s to standard output.
	int sum = a + b + c;
	cout << sum << ' ' << s << endl;

	return 0;
}
