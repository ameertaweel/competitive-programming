#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){

	constexpr int n = 5;

	int row, column;
	for(int r = 0; r < n; r++){
		for(int c = 0; c < n; c++){
			int cell;
			cin >> cell;
			if(cell) row = r, column = c;
		}
	}

	constexpr int center = n / 2;

	cout << (abs(row - center) + abs(column - center));

	return 0;
}
