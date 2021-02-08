#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n;
	cin >> n;

	int x = 0;
	while(n--){
		string statement;
		cin >> statement;
		if(statement == "++X" || statement == "X++") x++;
		else x--;
	}

	cout << x;

	return 0;
}
