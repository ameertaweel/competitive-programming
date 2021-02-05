#include <iostream>
#include <vector>
#include <queue>

using namespace std;

int main(){
	long t;
	cin >> t;

	while(t--){
		long n, m;
		cin >> n >> m;

		vector<long> currentPaints(n);
		for(auto& paint : currentPaints){
			cin >> paint;
		}

		vector<long> wantedPaints(n);
		for(auto& paint : wantedPaints){
			cin >> paint;
		}

		vector<long> paintersPaints(m);
		for(auto& paint : paintersPaints){
			cin >> paint;
		}

		vector<vector<long>> paintedRight(n + 1);
		vector<queue<long>> needPaint(n + 1);
		vector<long> hangingPainters;

		// Populate first two vectors
		for(long i = 0; i < n; i++){
			long a = currentPaints[i];
			long b = wantedPaints[i];

			if(a == b) paintedRight[a].push_back(i);
			else needPaint[b].push(i);
		}

		string indices;

		for(long j = 0; j < m; j++){
			long color = paintersPaints[j];
			bool isNeeded = !needPaint[color].empty();
			bool isPainted = !paintedRight[color].empty();

			long paintAt;
			if(isNeeded){
				paintAt = needPaint[color].front();
				needPaint[color].pop();
				paintedRight[color].push_back(paintAt);
			} else if(isPainted){
				paintAt = paintedRight[color][0];
			}

			if(isNeeded || isPainted){
				if(!hangingPainters.empty()){
					for(auto& painter : hangingPainters){
						indices += to_string(paintAt + 1);
						indices += ' ';
					}
					hangingPainters.clear();
				}
				indices += to_string(paintAt + 1);
				indices += ' ';
			} else {
				hangingPainters.push_back(color);
			}
		}

		if(!hangingPainters.empty()){
			cout << "NO\n";
			continue;
		}

		bool isNeedPaintingRemaining = false;
		for(auto& color : needPaint){
			if(!color.empty()){
				isNeedPaintingRemaining = true;
				break;
			}
		}
		if(isNeedPaintingRemaining){
			cout << "NO\n";
			continue;
		}

		cout << "YES\n";
		indices.pop_back();
		cout << indices << "\n";
	}

	return 0;
}
