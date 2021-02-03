#include <iostream>
#include <vector>

using namespace std;

int main(){
	int N;
	cin >> N;

	vector<int> A(N);
	for(int& a : A){
		int b;
		cin >> b;
		a = b;
	}

	vector<vector<int>> pairs(N, vector<int>(N, -1));
	int inversionNumber = 0;
	// Initial run, calculate the inversion number of A
	for(int i = 0; i < N - 1; i++){
			int a = A[i];
		for(int j = i + 1; j < N; j++){
			int b = A[j];
			int isLarger = a > b;
			pairs[a][b] = isLarger;
			inversionNumber += isLarger;
		}
	}

	cout << inversionNumber << "\n";

	// Run for all other transitions
	for(int i = 0; i < N - 1; i++){
		int a = A[i];
		for(int j = 0; j < N; j++){
			int b = A[j];
			int pair = pairs[a][b];
			// There is no such pair if a == -1
			if(pair == -1) continue;

			int isLarger = b > a;
			pairs[b][a] = isLarger;
			pairs[a][b] = -1;
			if(!pair && isLarger) inversionNumber++;
			else if(pair && !isLarger) inversionNumber--;
		}
		cout << inversionNumber << "\n";
	}

	return 0;
}
