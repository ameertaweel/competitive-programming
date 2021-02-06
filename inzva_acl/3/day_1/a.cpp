#include <iostream>

using namespace std;

int main(){
	string s;
	cin >> s;

	for(char c : s){
		long encryptedNum = ((long) c) - 65;
		// the A will be decrypted to Z
		long originalNum = (encryptedNum ? encryptedNum - 1 : 25) + 65;
		char originalChar = static_cast<char>(originalNum);
		cout << originalChar;
	}

	return 0;
}
