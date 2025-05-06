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


// macros
#define MAKE_UNSIGNED(n) (static_cast<make_unsigned<decltype(n)>::type>(n))
#define POPCOUNT(n) (__builtin_popcountll(MAKE_UNSIGNED(n)))
#define HAS_ODD_PARITY(n) (__builtin_parityll(MAKE_UNSIGNED(n)))
#define HAS_EVEN_PARITY(n) (!HAS_ODD_PARITY(n))
#define CLZ(n) (__builtin_clzll(MAKE_UNSIGNED(n)))
#define CTZ(n) (__builtin_ctzll(MAKE_UNSIGNED(n)))

il calc_cost(il a, il b) {
	return abs(a - b);
}

il find_lowest_cost_connection(il grade, vector<pil> row) {
	il min = -1;
	for (auto c : row) {
		il cost = calc_cost(grade, c.first);
		if (cost < min || min == -1) min = cost;
	}
	return min;
}

int main() {
	// fast io
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	il t;
	cin >> t;

	while (t--) {
		il n;
		cin >> n;

		vector<pil> a(n), b(n);
		for (il i = 0; i < n; i++) {
			cin >> a[i].first;
			a[i].second = i + 1;
		}
		for (il i = 0; i < n; i++) {
			cin >> b[i].first;
			b[i].second = i + 1;
		}

		il la1 = find_lowest_cost_connection(a[0].first, b);
		il lan = find_lowest_cost_connection(a[n - 1].first, b);
		il lb1 = find_lowest_cost_connection(b[0].first, a);
		il lbn = find_lowest_cost_connection(b[n - 1].first, a);

		il a1b1 = calc_cost(a[0].first, b[0].first);
		il a1bn = calc_cost(a[0].first, b[n - 1].first);
		il anb1 = calc_cost(a[n - 1].first, b[0].first);
		il anbn = calc_cost(a[n - 1].first, b[n - 1].first);

		vil choices = {
			// 4 Connections
			la1 + lan + lb1 + lbn,
			// 2 Connections
			a1b1 + anbn,
			a1bn + anb1,
			// 3 Connections
			a1b1 + lan + lbn,
			a1bn + lan + lb1,
			anb1 + la1 + lbn,
			anbn + la1 + lb1
		};

		sort(choices.begin(), choices.end());

		cout << choices[0] << "\n";
	}

	return 0;
}
