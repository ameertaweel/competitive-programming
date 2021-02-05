#include <iostream>

using namespace std;

int main(){
	long t;
	cin >> t;

	while(t--){
		long px, py;
		cin >> px >> py;
		string s;
		cin >> s;

		long u, d, l, r;
		u = d = l = r = 0;

		for(char c : s){
			switch(c){
				case 'U':
					u++;
					break;
				case 'D':
					d++;
					break;
				case 'L':
					l++;
					break;
				case 'R':
					r++;
					break;
			}
		}

		long currentX = r - l;
		long currentY = u - d;

		long diffX = currentX - px;
		long diffY = currentY - py;

		if(diffX > 0 && r < diffX){
			cout << "NO\n";
			continue;
		} else if(diffX < 0 && l < -diffX){
			cout << "NO\n";
			continue;
		}

		if(diffY > 0 && u < diffY){
			cout << "NO\n";
			continue;
		} else if(diffY < 0 && d < -diffY){
			cout << "NO\n";
			continue;
		}

		cout << "YES\n";
	}

	return 0;
}
