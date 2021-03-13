/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out && ./out < in
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t
#define ll long long

#define FAST_IO \
ios::sync_with_stdio(0);\
cin.tie(0);\
cout.tie(0);

int main(){
	FAST_IO;

	intfl a, b, w;
	cin >> a >> b >> w;

	w *= 1000;

	vector<intfl> max(w / a, a);
	intfl max_sum = (w / a) * a;

	intfl i = 0;
	while(max_sum < w){
		if((unsigned) i == max.size()) i = 0;
		if(max[i] + 1 <= b) max[i]++, max_sum++, i++;
		else {
			cout << "UNSATISFIABLE";
			return 0;
		}
	}

	vector<intfl> min(w / b, b);
	intfl min_sum = (w / b) * b;

	if(min_sum < w){
		min.push_back(a);
		min_sum += a;
	}
	intfl last_element = min.size() - 1;
	while(min_sum < w){
		min[last_element]++, min_sum++;
	}

	cout << min.size() << " " << max.size();

	return 0;
}
