#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n;
	cin >> n;

	vector<string> words(n);
	for(string& word : words){
		cin >> word;
		int len = word.length();
		if(len > 10) word = word[0] + to_string(len - 2) + word[len - 1];
	}

	for(string& word : words){
		cout << word << "\n";
	}

	return 0;
}
