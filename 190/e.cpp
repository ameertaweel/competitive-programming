#include <iostream>
#include <vector>
#include <queue>
#include <map>

using namespace std;

int main(){
	// Read input
	long N, M;
	cin >> N >> M;

	vector<vector<long>> graph(N);
	while(M--){
		long a, b;
		cin >> a >> b;
		a--, b--;

		graph[a].push_back(b);
		graph[b].push_back(a);
	}

	long K;
	cin >> K;

	vector<long> importantVerticesIndexes(K);
	for(auto& c : importantVerticesIndexes){
		cin >> c;
		c--;
	}

	// Use breadth search first to build find the distances
	vector<vector<long>> distanceMatrix(K, vector<long>(K, 0));

	for(long i = 0; i < K; i++){
		// Distances from each important vertex
		vector<long> dist(N, N);

		auto vertexIndex = importantVerticesIndexes[i];
		auto vertex = graph[vertexIndex];

		dist[vertexIndex] = 0;

		queue<int> verticesToTraverse;
		verticesToTraverse.push(vertexIndex);

		while(!verticesToTraverse.empty()){
			auto vi = verticesToTraverse.front();
			auto v = graph[vi];
			verticesToTraverse.pop();

			for(auto connected : v){
				if(dist[connected] > dist[vertexIndex] + 1){
					verticesToTraverse.push(connected);
					dist[connected] = dist[vi] + 1;
				}
			}
		}

		for(long j = 0; j < K; j++){
			distanceMatrix[i][j] = dist[importantVerticesIndexes[j]];
		}
	}

	// TODO: Remove
	for(auto i : distanceMatrix){
		for(auto j : i){
			cout << j << " ";
		}
		cout << endl;
	}

	return 0;
}
