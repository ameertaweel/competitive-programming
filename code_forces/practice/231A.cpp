#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n;
	cin >> n;

	int count = 0;

	while(n--){
		int friend1, friend2, friend3;
		cin >> friend1 >> friend2 >> friend3;
		count += (friend1 + friend2 + friend3) > 1;
	}

	cout << count;

	return 0;
}
