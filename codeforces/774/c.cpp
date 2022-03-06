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

int main() {
	// fast io
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	il max = 1e12;

	il t;
	cin >> t;

	vil factorials;
	for (il i = 4, f = 6; f < max; f *= i, i++) factorials.push_back(f);

	vector<pil> sums(1 << factorials.size());
	sums[0] = {0, 0};
	for (il mask = 1; mask < (1 << factorials.size()); mask++) {
		il first_bit = CTZ(mask);
		sums[mask].first = sums[mask ^ (1 << first_bit)].first + factorials[first_bit];
		sums[mask].second = POPCOUNT(mask);
	}

	while (t--) {
		il n;
		cin >> n;

		il result = POPCOUNT(n);

		for (auto i : sums) {
			if (i.first > n) continue;
			result = min(result, i.second + POPCOUNT(n - i.first));
		}

		cout << result << "\n";
	}

	return 0;
}
