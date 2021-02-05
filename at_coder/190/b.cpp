#include <iostream>

using namespace std;

int main(){
	// Read input
	int n, s, d;
	cin >> n >> s >> d;

	bool canDoDamage = false;

	for(int i = 0; i < n; i++){
		// Read input
		int time, power;
		cin >> time >> power;

		if(time < s && power > d){
			canDoDamage = true;
		}
	}

	cout << (canDoDamage ? "Yes" : "No");

	return 0;
}
