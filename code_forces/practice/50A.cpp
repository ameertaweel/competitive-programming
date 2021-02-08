#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n, m;
	cin >> n >> m;

	int total_tiles = n * m;

	if(total_tiles % 2 == 0) cout << total_tiles / 2;
	else cout << (total_tiles - 1) / 2;

	return 0;
}
