#include <iostream>
#include <vector>

using namespace std;

int main(){
	long h, w;
	cin >> h >> w;

	vector<vector<bool>> grid(w, vector<bool>(h));
	for(auto& row : grid){
		for(int i = 0; i < h; i++){
			char paint;
			cin >> paint;
			row[i] = paint == '.' ? 1 : 0;
		}
	}

	return 0;
}
