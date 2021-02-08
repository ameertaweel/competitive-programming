#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n, k;
	cin >> n >> k;

	vector<int> scores(n);
	for(int& score : scores){
		cin >> score;
	}

	int k_score = scores[k - 1];

	int count = 0;
	for(int& score: scores){
		if(score >= k_score && score > 0) count++;
		else break;
	}

	cout << count;

	return 0;
}
