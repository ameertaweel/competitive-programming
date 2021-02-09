#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <regex>

using namespace std;

int main(){
	string program;
	cin >> program;

	cout << (regex_match(program, regex(".*(H|Q|9).*")) ? "YES" : "NO");

	return 0;
}
