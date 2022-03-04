#include <bits/stdc++.h>

using namespace std;

// type definitions
typedef int_fast8_t ib;
typedef int_fast16_t is;
typedef int_fast32_t ii;
typedef int_fast64_t il;
typedef uint_fast8_t ub;
typedef uint_fast16_t us;
typedef uint_fast32_t ui;
typedef uint_fast64_t ul;
typedef vector<ib> vib;
typedef vector<is> vis;
typedef vector<ii> vii;
typedef vector<il> vil;
typedef vector<ub> vub;
typedef vector<us> vus;
typedef vector<ui> vui;
typedef vector<ul> vul;
typedef pair<ib, ib> pib;
typedef pair<is, is> pis;
typedef pair<ii, ii> pii;
typedef pair<il, il> pil;
typedef pair<ub, ub> pub;
typedef pair<us, us> pus;
typedef pair<ui, ui> pui;
typedef pair<ul, ul> pul;

int main() {
	// fast io
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	il max = 1e12;

	il t;
	cin >> t;

	// 12 factorials
	il factorials[] = {6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 6227020800, 87178291200};

	vil combi;
	vil combi_n;
	for (il i = 1; i < (1 << 12); i++) {
		il sum = 0;
		il sum_n = 0;
		for (il j = 0; j < 12; j++) {
			if (i & (1 << j)) {
				sum += factorials[j];
				sum_n++;
			}
		}
		if (sum <= max) {
			combi.push_back(sum);
			combi_n.push_back(sum_n);
		}
	}

	while (t--) {
		il n;
		cin >> n;

		il gain = 0;
		for (il i = 0; i < (il) combi.size(); i++) {
			il c = combi[i];
			if (c > n) continue;
			il shared = 0;
			for (il j = 1; j < max; j *= 2){
				if (c & n & j) shared++;
			}
			il cur_gain = shared - combi_n[i];
			if (cur_gain > gain) {
				gain = cur_gain;
			}
		}
		il answ = 0;
		for (il i = 1; i <= max; i *= 2){
			if (n & i) answ++;
		}
		answ -= gain;

		cout << answ << "\n";
	}

	return 0;
}
