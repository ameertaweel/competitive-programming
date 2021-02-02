#include <iostream>
#include <vector>
#include <queue>

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

	if(K == 1){
		cout << 1;
		return 0;
	}

	long INF = N * K;

	vector<long> importantVerticesIndexes(K);
	for(auto& c : importantVerticesIndexes){
		cin >> c;
		c--;
	}

	// Use breadth search first to build find the distances
	vector<vector<long>> distanceMatrix(K, vector<long>(K));

	for(long i = 0; i < K; i++){
		// Distances from each important vertex
		vector<long> dist(N, INF);

		auto vertexIndex = importantVerticesIndexes[i];
		auto vertex = graph[vertexIndex];

		dist[vertexIndex] = 0;

		queue<long> verticesToTraverse;
		verticesToTraverse.push(vertexIndex);

		while(!verticesToTraverse.empty()){
			auto vi = verticesToTraverse.front();
			auto v = graph[vi];
			verticesToTraverse.pop();

			for(auto connected : v){
				if(dist[connected] > dist[vi] + 1){
					verticesToTraverse.push(connected);
					dist[connected] = dist[vi] + 1;
				}
			}
		}

		for(long j = 0; j < K; j++){
			distanceMatrix[i][j] = dist[importantVerticesIndexes[j]];
		}
	}

	// Use Dynamic Programming (DP) and Bitmasks to find the shortest path, if exists
	// N * K here is infinity, because a real path can't be that long
	vector<vector<long>> dp(1 << K, vector<long>(K, INF));

	// We do not care about the bitmask 0^K (where ^ means repetition), so we start enumerating from 1
	// Initialize the states 2^i for i between 0 and K - 1 to be 1 (because we only added one stone so far)
	for(long i = 0; i < K; i++){
		dp[1 << i][K - i - 1] = 1;
	}

	// Calculate the rest of the paths (permutations)
	// The - 1 in (1 << K) - 1 is because we do not need to loop over the mask 1*K (where ^ means repetition)
	for(long mask = 1; mask < (1 << K) - 1; mask++){
		// Loop over the mask and find all possible last elements (if the bit is set (equal to 1))
		for(long last = 0; last < K; last++){
			long lastBitMask = 1 << (K - last - 1);
			// Skip if the bit is 0
			if(!(mask & lastBitMask)) continue;
			// Loop over all the possible continuations
			for(long next = 0; next < K; next++){
				long nextBitMask = 1 << (K - next - 1);
				// Skip if the bit is 1, because the element does not need to be added twice
				if(mask & nextBitMask) continue;
				long nextBitAddedMask = mask | nextBitMask;
				long newDistance = dp[mask][last] + distanceMatrix[last][next];
				long oldDistance = dp[nextBitAddedMask][next];

				dp[nextBitAddedMask][next] = newDistance < oldDistance ? newDistance : oldDistance;
			}
		}
	}

	// Loop over the complete path (paths of form 1 ^ K (where ^ means repetition)) and find the shortest path
	vector<long>& completePaths = dp[(1 << K) - 1];
	long min = INF;
	for(auto i : completePaths){
		if(i < min) min = i;
	}

	// Display result
	cout << (min < INF) ? min : -1;

	return 0;
}
