#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

int main(){
	long N;
	cin >> N;

	vector<long> logs(N);
	for(auto& log : logs){
		cin >> log;
	}

	sort(logs.begin(), logs.end());

	long count = 0;
	for(long i = 0; i < N - 2; i++){
		long logI = logs[i];
		for(long j = i + 1; j < N - 1; j++){
			long logJ = logs[j];
			for(long k = j + 1; k < N; k++){
				long logK = logs[k];
				if(logI + logJ > logK) count++;
			}
		}
	}
	cout << count;

	return 0;
}
