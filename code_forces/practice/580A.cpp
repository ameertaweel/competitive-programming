#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	long n;
	cin >> n;

	long last = -1;
	long current_streak = 0;
	long longest_streak = 0;
	while(n--){
		long daily_profit;
		cin >> daily_profit;

		if(daily_profit >= last) current_streak++;
		else current_streak = 1;

		last = daily_profit;

		if(current_streak > longest_streak) longest_streak = current_streak;
	}

	cout << longest_streak;

	return 0;
}
