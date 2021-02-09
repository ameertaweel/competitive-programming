#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <bitset>

using namespace std;

int main(){
	long n;
	cin >> n;

	auto total_sum = 0.0;

	vector<int> coins(n);
	for(auto& coin : coins){
		cin >> coin;
		total_sum += coin;
	}

	auto half_money = total_sum / 2;

	sort(coins.begin(), coins.end());

	// We minimize the number of coins by taking the larger ones
	auto my_money = 0;
	auto my_coins = 0;
	for(auto i = coins.rbegin(); i != coins.rend(); i++){
		my_money += *i;
		my_coins++;
		if(my_money > half_money) break;
	}

	cout << my_coins;

	return 0;
}
