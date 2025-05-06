// Link To Problem:
// https://codeforces.com/contest/2030/problem/C

#include <bits/stdc++.h>

using namespace std;

// type definitions
using i08 = int_fast8_t;
using i16 = int_fast16_t;
using i32 = int_fast32_t;
using i64 = int_fast64_t;
using u08 = uint_fast8_t;
using u16 = uint_fast16_t;
using u32 = uint_fast32_t;
using u64 = uint_fast64_t;

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

#define CHAR_TO_BOOl(c) ((c) == '1')

int main() {
	// Fast I/O
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	i64 t;
	cin >> t;

	outer: for (i64 i = 0; i < t; i++) {
		i64 n;
		cin >> n;

		string s;
		cin >> s;

		if (CHAR_TO_BOOl(s[0])) {
			cout << "YES" << endl;
			goto continue_outer;
		}

		if (CHAR_TO_BOOl(s[n - 1])) {
			cout << "YES" << endl;
			goto continue_outer;
		}

		for (i64 j = 0; j < n - 1; j ++) {
			if (CHAR_TO_BOOl(s[j]) && CHAR_TO_BOOl(s[j + 1])) {
				cout << "YES" << endl;
				goto continue_outer;
			}
		}

		cout << "NO" << endl;

		continue_outer:;
	}

	return 0;
}
