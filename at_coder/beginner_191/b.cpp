#include <iostream>
#include <vector>

using namespace std;

int main(){
	long n, x;
	cin >> n >> x;

	vector<long> toPrint;
	for(long i = 0; i < n; i++){
		long ai;
		cin >> ai;
		if(ai != x) toPrint.push_back(ai);
	}

	for(auto& i : toPrint){
		cout << i << " ";
	}

	return 0;
}
