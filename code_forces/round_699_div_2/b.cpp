#include <iostream>
#include <vector>

using namespace std;

int main(){
	long t;
	cin >> t;

	while(t--){
		long n, k;
		cin >> n >> k;

		vector<long> h(n);
		for(long& hi : h){
			cin >> hi;
		}

		long i = 0;
		long j = 0;
		long out;
		while(i < n && j < k){
			if(i == n - 1){
				out = -1;
				j++;
				continue;
			}
			long current = h[i];
			long next = h[i + 1];

			if(current >= next){
				i++;
				continue;
			}
			// Here current will be < next
			h[i]++;
			out = i + 1;
			j++;
			i--;
		}
		cout << out << "\n";
	}

	return 0;
}
