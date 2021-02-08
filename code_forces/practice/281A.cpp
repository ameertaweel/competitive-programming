#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	string word;
	cin >> word;

	word[0] = toupper(word[0]);

	cout << word;

	return 0;
}
