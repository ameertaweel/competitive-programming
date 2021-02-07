#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>

using namespace std;

int main(){
	long t;
	cin >> t;

	while(t--){
		long double A, B;
		long n;
		cin >> A >> B >> n;

		vector<pair<long double, long double>> monsters(n);
		for(auto& monster : monsters){
			cin >> monster.first;
		}
		for(auto& monster : monsters){
			cin >> monster.second;
		}

		sort(monsters.begin(), monsters.end(), [&A](const auto& lhs, const auto& rhs) {
			// The amount of damage the hero takes is constant
			// We just need to make the strongest hit come last
			// So he might kill the monster and die
			return lhs.first < rhs.first;
		});

		bool state = true;
		for(auto& monster : monsters){
			long heroNeededFights = ceil(monster.second / A);
			long monsterNeededFights = ceil(B / monster.first);

			if(monsterNeededFights < heroNeededFights){
				state = false;
				break;
			}

			monster.second -= heroNeededFights * A;
			B -= heroNeededFights * monster.first;
		}

		cout << (state ? "YES" : "NO") << "\n";
	}

	return 0;
}
