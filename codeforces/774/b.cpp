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

	il t;
	cin >> t;

	while (t--) {
		il n;
		cin >> n;

		vil v(n);
		for (il i = 0; i < n; i++) {
			cin >> v[i];
		}
		sort(v.begin(), v.end());
		
		il start = 1;
		il end = n - 1;

		il red_sum = 0;
		il blue_sum = v[0];

		bool is_yes = false;

		while (start < end) {
			red_sum += v[end--];
			blue_sum += v[start++];

			if (red_sum > blue_sum) {
				cout << "YES\n";
				is_yes = true;
				break;
			}
		}

		if (!is_yes) cout << "NO\n";
	}

	return 0;
}
