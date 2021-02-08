#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

using namespace std;

int main(){
	int ones, twos, threes;
	ones = twos = threes = 0;

	string math_exp;
	cin >> math_exp;
	for(char& c : math_exp){
		switch(c){
			case '1': ones++; break;
			case '2': twos++; break;
			case '3': threes++; break;
		}
	}

	string new_math_exp = math_exp;
	for(auto i = 0LU; i < new_math_exp.length(); i += 2){
		if(ones) ones--, new_math_exp[i] = '1';
		else if(twos) twos--, new_math_exp[i] = '2';
		else if(threes) threes--, new_math_exp[i] = '3';
	}

	cout << new_math_exp;

	return 0;
}
