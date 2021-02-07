#include <iostream>

using namespace std;

int main(){
	long t;
	cin >> t;

	while(t--){
		string s;
		cin >> s;

		string newS;
		bool isAliceTurn = true;
		for(char c : s){
			if(isAliceTurn){
				if(c == 'a') newS += 'b';
				else newS += 'a';
				isAliceTurn = false;
			} else {
				if(c == 'z') newS += 'y';
				else newS += 'z';
				isAliceTurn = true;
			}
		}

		cout << newS << "\n";
	}

	return 0;
}
