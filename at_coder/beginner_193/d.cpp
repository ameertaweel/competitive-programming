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

	intfl k;
	string s, t;

	cin >> k >> s >> t;

	vector<intfl> cards(9, k);
	vector<intfl> a_cards(9, 0);
	vector<intfl> b_cards(9, 0);

	for(auto c : s){
		if(c == '#') break;
		intfl card_val = c - '0';
		cards[card_val - 1]--;
		a_cards[card_val - 1]++;
	}

	for(auto c : t){
		if(c == '#') break;
		intfl card_val = c - '0';
		cards[card_val - 1]--;
		b_cards[card_val - 1]++;
	}

	vector<intfl> a_scores(9, 0);
	vector<intfl> b_scores(9, 0);

	for(intfl i = 0; i < 9; i++){
		a_cards[i]++;
		for(intfl j = 0; j < 9; j++){
			a_scores[i] += (j + 1) * pow(10, a_cards[j]);
		}
		a_cards[i]--;
	}

	for(intfl i = 0; i < 9; i++){
		b_cards[i]++;
		for(intfl j = 0; j < 9; j++){
			b_scores[i] += (j + 1) * pow(10, b_cards[j]);
		}
		b_cards[i]--;
	}

	intfl remaining_cards = 9 * k - 8;
	long double total_probability = 0;
	for(intfl i = 0; i < 9; i++){
		// Make sure a card with kind (i) is remaining
		if(!cards[i]) continue;
		long double probability_a = (double) cards[i] / remaining_cards;
		cards[i]--;
		for(intfl j = 0; j < 9; j++){
			// Make sure a card with kind (j) is remaining
			if(!cards[j]) continue;
			if(a_scores[i] <= b_scores[j]) continue;
			long double probability_b =	(double) cards[j] / (remaining_cards - 1);
			long double probability = probability_a * probability_b;
			total_probability += probability;
		}
		cards[i]++;
	}

	cout << total_probability;

	return 0;
}
