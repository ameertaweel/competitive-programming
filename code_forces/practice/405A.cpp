#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int n;
	cin >> n;

	vector<int> columns(n);
	for(auto& column : columns){
		cin >> column;
	}

	sort(columns.begin(), columns.end());

	for(auto& column : columns){
		cout << column << " ";
	}

	return 0;
}
