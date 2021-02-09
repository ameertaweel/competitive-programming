#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	// This is a geometric series problem
	long t;
	cin >> t;

	while(t--){
		long n;
		cin >> n;

		// 2^30 is larger than 10^9 so we stop at k = 29
		// 2*k is always of the form (1 repeated k times);
		for(auto k = 2; k < 30; k++){
			auto two_power_k_minus_one = (1 << k) - 1;
			if(n % two_power_k_minus_one == 0){
				cout << (n / two_power_k_minus_one) << "\n";
				break;
			}
		}
	}

	return 0;
}
