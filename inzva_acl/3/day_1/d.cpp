#include <iostream>
#include <vector>

using namespace std;

const char MOVE = 'M';
const char ROTATE = 'C';

const int UP = 0;
const int RIGHT = 1;
const int DOWN = 2;
const int LEFT = 3;

string trimStartRotationsAndFlip(string);
pair<string, long> removePowersOfTwoRotations(string);
void move(long&, long&, int);
void rotate(int&, bool);

int main(){
	string directions;
	cin >> directions;

	long distX, distY;
	cin >> distX >> distY;

	auto parsed = removePowersOfTwoRotations(trimStartRotationsAndFlip(trimStartRotationsAndFlip(directions)));

	directions = parsed.first;
	auto numOfRotations = parsed.second;

	// The initial direction is to the right (from a discussion in the forum)
	// Try all combinations of rotations
	for(long mask = 0; mask < (1 << numOfRotations); mask++){
		int direction = RIGHT;
		long rotationIndex = 0;
		long x, y;
		x = 1;
		y = 0;
		for(char d : directions){
			if(d == MOVE) move(x, y, direction);
			else {
				long extractBitMask = 1 << rotationIndex;
				bool bit = mask & extractBitMask;
				rotate(direction, bit);
				rotationIndex++;
			}
		}
		if(x == distX && y == distY){
			cout << "YES";
			return 0;
		}
	}

	cout << "NO";
	return 0;
}

string trimStartRotationsAndFlip(string directions){
	// Remove the C's from the beginning of the directions as they have no effect
	// Flip the string so it can be processed again to remove C's from the end
	string trimStart;
	bool foundFirstMoveCommand = false;
	for(char command : directions){
		if(foundFirstMoveCommand) trimStart = command + trimStart;
		else if(command == MOVE){
			foundFirstMoveCommand = true;
			trimStart = command + trimStart;
		}
	}
	return trimStart;
}

pair<string, long> removePowersOfTwoRotations(string directions){
	// Replace any even number of C's greater than 2 with 2 C's as they are equivalent
	// Returns the new string and the count of rotation commands
	string shorter;
	long numOfRotations = 0;
	long totalNumOfRotations = 0;

	for(char c : directions){
		if(c == ROTATE) {
			numOfRotations++;
			continue;
		}
		if(numOfRotations > 3){
			if(numOfRotations % 2 == 0) numOfRotations = 2;
			else numOfRotations = 3;
			totalNumOfRotations += numOfRotations;
		}
		for(long i = 0; i < numOfRotations; i++){
			shorter += ROTATE;
		}
		numOfRotations = 0;
		shorter += MOVE;
	}

	pair<string, long> result;
	result.first = shorter;
	result.second = totalNumOfRotations;

	return result;
}

void move(long& x, long& y, int direction){
	switch(direction){
		case UP:
			y++;
			break;
		case DOWN:
			y--;
			break;
		case RIGHT:
			x++;
			break;
		case LEFT:
			x--;
			break;
	}
}

void rotate(int& direction, bool bit){
	if(bit) direction++;
	else direction--;

	if(direction < 0) direction = 4;
	else if (direction > 4) direction = 0;
}
