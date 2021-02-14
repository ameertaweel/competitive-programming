#include <iostream>
#include <vector>

using namespace std;

int main(){
	// Read input start
	int n, m;
	cin >> n >> m;

	vector<pair<int, int>> conditions(m);
	for(auto& condition : conditions){
		int a, b;
		cin >> a >> b;
		condition.first = a - 1;
		condition.second = b - 1;
	}

	int k;
	cin >> k;

	vector<pair<int, int>> choices(k);
	for(auto& choice : choices){
		int c, d;
		cin >> c >> d;
		choice.first = c - 1;
		choice.second = d - 1;
	}
	// Read input end

	// We need to consider the 2^k permutations
	// We will use a bit mask
	int maxCount = 0;
	for(int i = 0; i < (1 << k); i++){
		// Create empty list of dishes
		vector<int> dishes(n);

		// The first bit from the right corresponds to the first person
		for(int j = 0; j < k; j++){
			// Shift j places to the left
			int bit = ((1 << j) & i) >= 1 ? 1 : 0;

			// If the bit is 0, then the person chose the first option
			if(!bit) dishes[choices[j].first]++;
			else dishes[choices[j].second]++;
		}

		// Calculate score
		int count = 0;
		for(auto& condition : conditions){
			if(dishes[condition.first] && dishes[condition.second]) count++;
		}
		if(count > maxCount) maxCount = count;
	}

	cout << maxCount;

	return 0;
}
