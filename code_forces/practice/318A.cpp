#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int64_t n, k;
	cin >> n >> k;

	int64_t center = n % 2 == 0 ? n / 2 : n / 2 + 1;

	cout << (k > center ? (k - center) * 2 : k * 2 - 1);

	return 0;
}
