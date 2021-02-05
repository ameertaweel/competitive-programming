// I failed at sorting 5 items in maximum of 7 steps.
// There is an algorithm called Merge-Insertion Sort that can do that.

#include <iostream>
#include <vector>

using namespace std;

class Ball{
public:
	char value;
	Ball(char value) :value{value} {}

	bool isSmaller(Ball otherBall){
		cout << "? " << this -> value << " " << otherBall.value << endl;

		char sign;
		cin >> sign;

		return sign == '<';
	}
};

vector<Ball> mergeTwoSortedBallVectors(vector<Ball> ballsVectorA, vector<Ball> ballsVectorB){
	vector<Ball> merged;

	long unsigned int i = 0;
	long unsigned int j = 0;

	while(i < ballsVectorA.size() && j < ballsVectorB.size()){
		if(ballsVectorA[i].isSmaller(ballsVectorB[j])){
			merged.push_back(ballsVectorA[i]);
			i++;
		} else {
			merged.push_back(ballsVectorB[j]);
			j++;
		}
	}

	while(i < ballsVectorA.size()){
		merged.push_back(ballsVectorA[i]);
		i++;
	}

	while(j < ballsVectorB.size()){
		merged.push_back(ballsVectorB[j]);
		j++;
	}

	return merged;
}

vector<Ball> mergeSortBalls(vector<Ball> balls){
	int length = balls.size();

	if(length < 2){
		return balls;
	}

	int halfLength = length / 2;

	vector<Ball> firstHalfOfBalls;
	vector<Ball> secondHalfOfBalls;

	for(int i = 0; i < halfLength; i++){
		firstHalfOfBalls.push_back(balls[i]);
	}

	for(int i = halfLength; i < length; i++){
		secondHalfOfBalls.push_back(balls[i]);
	}

	return mergeTwoSortedBallVectors(mergeSortBalls(firstHalfOfBalls), mergeSortBalls(secondHalfOfBalls));
}

int main(){
	vector<char> alphabet = {'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'};

	// Get values of N and Q
	int N, Q;
	cin >> N >> Q;

	vector<Ball> balls;
	for(int i = 0; i < N; i++){
		balls.push_back(Ball(alphabet[i]));
	}

	vector<Ball> sortedBalls = mergeSortBalls(balls);

	cout << "! ";
	for(int i = 0; i < N; i++){
		cout << sortedBalls[i].value;
	}
	cout << endl;

	return 0;
}
