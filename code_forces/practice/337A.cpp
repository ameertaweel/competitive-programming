#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n, m;
	cin >> n >> m;

	vector<int> puzzles(m);
	for(auto& puzzle : puzzles){
		cin >> puzzle;
	}

	sort(puzzles.begin(), puzzles.end());

	int min_diff = 1000;
	for(int min = 0, max = n - 1; max < m; min++, max++){
		int current_diff = puzzles[max] - puzzles[min];
		if(current_diff < min_diff) min_diff = current_diff;
	}

	cout << min_diff;

	return 0;
}
