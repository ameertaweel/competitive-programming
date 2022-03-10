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

	il n;
	cin >> n;

	vector<pil> ordered_points(n);
	set<pil> points;
	for (il i = 0, x, y; i < n; i++) {
		cin >> x >> y;
		ordered_points[i].first = x;
		ordered_points[i].second = y;
		points.insert({x, y});
	}

	vector<pil> directions = {{-1, -1}, {-1, 1}, {1, 1}, {1, -1}};
	for (auto p : ordered_points) {
		il direction = 0;
		il radius = 1;
		il sides = 0;
		il counter = 0;
		pil n = { p.first + 1, p.second };
		while (points.count({n.first, n.second})) {
			if (counter == radius) {
				counter = 0;
				direction++;
				sides++;
			}
			if (sides == 4) {
				radius++;
				sides = 0;
				n.first = p.first + radius;
				n.second = p.second;
				direction = 0;
				counter = 0;
			}
			n.first += directions[direction].first;
			n.second += directions[direction].second;
			counter++;
		}
		cout << n.first << " " << n.second << "\n";
	}

	return 0;
}
