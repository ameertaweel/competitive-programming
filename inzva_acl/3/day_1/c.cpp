#include <iostream>
#include <vector>
#include <queue>

using namespace std;

class City {
	public:
		long index;
		vector<long> connections;
		long comingFrom;
		bool visited;
};

int main(){
	long N, M;
	cin >> N >> M;

	vector<long> cities(N);
	for(auto& city : cities){
		long cityInput;
		cin >> cityInput;
		city = cityInput - 1;
	}

	vector<City> graph(N);
	for(long i = 0; i < M; i++){
		long cityAIndex, cityBIndex;
		cin >> cityAIndex >> cityBIndex;
		cityAIndex--, cityBIndex--;

		graph[cityAIndex].index = cityAIndex;
		graph[cityAIndex].connections.push_back(cityBIndex);
		graph[cityBIndex].index = cityBIndex;
		graph[cityBIndex].connections.push_back(cityAIndex);
	}

	queue<City*> citiesToCheck;
	// Start by visiting the capital
	auto& capital = graph[0];
	capital.comingFrom = -1;
	capital.visited = true;
	citiesToCheck.push(&capital);

	long counter = 0;
	while(!citiesToCheck.empty() && counter < N - 1){
		auto& city = citiesToCheck.front();
		citiesToCheck.pop();
		bool found = false;
		counter++;
		for(auto& connectedCityIndex : city -> connections){
			auto& connectedCity = graph[connectedCityIndex];
			if(connectedCityIndex == cities[counter] && !connectedCity.visited){
				connectedCity.comingFrom = city -> index;
				citiesToCheck.push(&connectedCity);
				found = true;
				connectedCity.visited = true;
				break;
			}
		}
		if(!found) {
			if(city -> comingFrom != -1){
				auto& previous = graph[city -> comingFrom];
				citiesToCheck.push(&previous);
				counter--;
			} else {
				cout << "No";
				return 0;
			}
		}
	}

	cout << "Yes";

	return 0;
}
