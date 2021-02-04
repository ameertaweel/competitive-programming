#include <iostream>
#include <vector>

using namespace std;

pair<long, vector<long>> merge(pair<long, vector<long>>, pair<long, vector<long>>);
pair<long, vector<long>> mergeSort(vector<long>&, long = -1, long = -1);

int main(){
	long N;
	cin >> N;

	vector<long> sequence(N);
	// use 1 as base index instead of 0
	for(long& element : sequence){
		long i;
		cin >> i;
		element = i + 1;
	}

	long inversionNumber = mergeSort(sequence).first;

	cout << inversionNumber << "\n";

	// run for all other transitions
	for(long i = 0; i < N - 1; i++){
		long k = sequence[i];
		// for each rotation (k is the element moved to the end of sequence):
		// we loose k - 1 inversion pairs
		// we gain n - k inversion pairs
		inversionNumber -= k - 1;
		inversionNumber += N - k;
		cout << inversionNumber << "\n";
	}

	return 0;
}

pair<long, vector<long>> mergeSort(vector<long>& sequence, long start, long end){
	start = start == -1 ? 0 : start;
	end = end == -1 ? sequence.size() : end;

	long length = end - start;

	pair<long, vector<long>> pair;
	if(length == 1){
		pair.first = 0;
		vector<long> sorted(1);
		sorted[0] = sequence[start];
		pair.second = sorted;
		return pair;
	} else if(length == 0){
		pair.first = 0;
		vector<long> sorted(0);
		pair.second = sorted;
		return pair;
	}

	long mid = start + length / 2;

	return merge(mergeSort(sequence, start, mid), mergeSort(sequence, mid, end));
}

pair<long, vector<long>> merge(pair<long, vector<long>> a, pair<long, vector<long>> b){
	long sizeA = a.second.size();
	long sizeB = b.second.size();

	pair<long, vector<long>> pair;
	pair.first += a.first + b.first;

	long i, j;
	i = j = 0;

	while(i < sizeA && j < sizeB){
		long ai = a.second[i];
		long bj = b.second[j];

		if(ai > bj){
			pair.second.push_back(bj);
			pair.first += sizeA - i;
			j++;
		} else {
			pair.second.push_back(ai);
			i++;
		}
	}

	while(i < sizeA){
		long ai = a.second[i];
		pair.second.push_back(ai);
		i++;
	}

	while(j < sizeB){
		long bj = b.second[j];
		pair.second.push_back(bj);
		j++;
	}

	return pair;
}
