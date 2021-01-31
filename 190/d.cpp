#include <iostream>
#include <vector>
#include <math.h>

using namespace std;

vector<long>* calculateFactors(long N){
	long twoN = 2 * N;
	// Return the factor pairs of n
	vector<long>* factors = new vector<long>;

	long sqrtOfN = sqrt(twoN);

	for(long i = 1; i <= sqrtOfN; i++){
		long q = twoN / i;
		if(i * q == twoN){
			factors -> push_back(i);
			if(i != q) factors -> push_back(q);
		}
	}

	return factors;
}

pair<bool, long>* isSequence(double N, double factor){
	pair<bool, long>* result = new pair<bool, long>;

	double a = N / factor - factor / 2 + 0.5;
	// If "a" is an integer, then this is a valid sequence.
	if(trunc(a) == a){
		result -> first = true;
		result -> second = a;
	}

	return result;
}

int main(){
	// Read input
	long N;
	cin >> N;

	// A sum of a sequence starting with a, and ending with a + n -1,
	// where n is the number of elements in the sequence is:
	// ((a + a + n - 1) * n) / 2
	// We want this to equal N
	vector<long>* factors = calculateFactors(N);

	vector<long> sequences;
	for(long factor : *factors){
		pair<bool, long>* result = isSequence(N, factor);
		if(result -> first) sequences.push_back(result -> second);
	}

	cout << sequences.size();

	return 0;
}
