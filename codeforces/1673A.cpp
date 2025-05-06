#include <bits/stdc++.h>

using namespace std;

// type definitions
using ib = int_fast8_t;
using is = int_fast16_t;
using ii = int_fast32_t;
using il = int_fast64_t;
using ub = uint_fast8_t;
using us = uint_fast16_t;
using ui = uint_fast32_t;
using ul = uint_fast64_t;
using vib = vector<ib>;
using vis = vector<is>;
using vii = vector<ii>;
using vil = vector<il>;
using vub = vector<ub>;
using vus = vector<us>;
using vui = vector<ui>;
using vul = vector<ul>;
using pib = pair<ib, ib>;
using pis = pair<is, is>;
using pii = pair<ii, ii>;
using pil = pair<il, il>;
using pub = pair<ub, ub>;
using pus = pair<us, us>;
using pui = pair<ui, ui>;
using pul = pair<ul, ul>;

// macros
#define MAKE_UNSIGNED(n) (static_cast<make_unsigned<decltype(n)>::type>(n))
#define POPCOUNT(n) (__builtin_popcountll(MAKE_UNSIGNED(n)))
#define HAS_ODD_PARITY(n) (__builtin_parityll(MAKE_UNSIGNED(n)))
#define HAS_EVEN_PARITY(n) (!HAS_ODD_PARITY(n))
#define CLZ(n) (__builtin_clzll(MAKE_UNSIGNED(n)))
#define CTZ(n) (__builtin_ctzll(MAKE_UNSIGNED(n)))
#define SORT_STD_INC(x) sort(x.begin(), x.end())
#define SORT_STD_DEC(x) sort(x.rbegin(), x.rend())
#define SORT_ARR_INC(x) sort(a, a + (sizeof a / sizeof a[0]))
#define SORT_ARR_DEC(x) sort(a, a + (sizeof a / sizeof a[0]), greater<>())
#define REVERSE_STD(x) reverse(x.begin(), x.end())
#define REVERSE_ARR(x) reverse(a, a + (sizeof a / sizeof a[0]))

int main() {
	// fast io
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	il t;
	cin >> t;

	while (t--) {
		string s;
		cin >> s;

		il total = 0;
		for (char c : s) total += c - 'a' + 1;

		if (s.length() % 2 == 0) {
			cout << "Alice " << total << "\n";
			continue;
		}

		il bob = min(s[0], s[s.length() - 1]) - 'a' + 1;
		il alice = total - bob;

		il diff = abs(alice - bob);
		cout << (alice > bob ? "Alice " : "Bob ") << diff << "\n";
	}

	return 0;
}
