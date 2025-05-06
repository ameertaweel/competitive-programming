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

	il t;
	cin >> t;

	tests: while (t--) {
		string s, c;
		cin >> s >> c;

		for (il i = 0; i < (il) s.length(); i += 2) {
			if (s[i] == c[0]) {
				cout << "YES" << "\n";
				goto tests;
			}
		}

		cout << "NO" << "\n";
	}

	return 0;
}
