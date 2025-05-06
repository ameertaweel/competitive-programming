// Link To Problem:
// https://codeforces.com/problemset/problem/967/B

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

int main() {
	// Fast I/O
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);

	i64 n, A, B;
	cin >> n >> A >> B;

	vector<u64> s(n);
	i64 S = 0;

	for (u64 i = 0; i < n; i++) {
		cin >> s[i];
		S += s[i];
	}

	i64 should_block_size = S - ((s[0] * A) / B);

	i64 blocked_size  = 0;
	i64 blocked_count = 0;

	// Sort in descending order (excluding first element)
	sort(s.rbegin(), s.rend() - 1);

	while (blocked_size < should_block_size) {
		blocked_size  += s[blocked_count + 1];
		blocked_count += 1;
	}

	cout << blocked_count << endl;

	return 0;
}
