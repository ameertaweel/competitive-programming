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
		il n;
		cin >> n;

		vector<vil> segments;
		vector<pil> deletions;
		vil current_segment = {};
		il current_segment_start = 0;
		for (il i = 0; i < n; i++) {
			il ai;
			cin >> ai;

			if (ai == 0) {
				if (current_segment.size()) {
					segments.push_back(current_segment);
					deletions.push_back({
						current_segment_start,
						n - i
					});
				}
				current_segment = {};
				current_segment_start = i + 1;
				continue;
			}
			current_segment.push_back(ai);
		}
		if (current_segment.size()) {
			segments.push_back(current_segment);
			deletions.push_back({
			   current_segment_start,
			   0
			});
		}

		if (!segments.size()) {
			cout << n << " 0\n";
			continue;
		}

		il max = 0;
		il max_index = -1;

		for (il i = 0; i < (il) segments.size(); i++) {
			auto s = segments[i];
			auto d = &deletions[i];

			il product = 1;
			for (auto num : s) product *= num;

			if (product < 0 && -product > max) {
				il start_two_loss = 0;
				il start_deletions = 0;
				for (il j = 0; j < (il) s.size(); j++) {
					if (s[j] == 2 || s[j] == -2) start_two_loss++;
					if(s[j] < 0) {
						start_deletions = j + 1;
						break;
					}
				}

				il end_two_loss = 0;
				il end_deletions = 0;
				for (il j = (il) s.size() - 1; j >= 0; j--) {
					if (s[j] == 2 || s[j] == -2) end_two_loss++;
					if(s[j] < 0) {
						end_deletions = n - j;
						break;
					}
				}

				il min_loss = min(start_two_loss, end_two_loss);
				if (min_loss == start_two_loss) d -> first += start_deletions;
				else d -> second += end_deletions;
				product *= -1;
				while (min_loss--) product /= 2;
			}

			if (product > max) {
				max = product;
				max_index = i;
			}
		}

		auto d = deletions[max_index];
		cout << d.first << " " << d.second << "\n";
	}

	return 0;
}
