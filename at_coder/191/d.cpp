#include <iostream>
#include <math.h>

using namespace std;

double dist(double, double, double, double);

int main(){
	double x, y, r;
	cin >> x >> y >> r;

	// Pares the upper right quarter of the circle by lines
	long length = r;
	long count = 0;

	while(length >= 0){
		if(dist(x, y,
	}

	return 0;
}

double dist(double x1, double y1, double x2, double y2){
	double deltaX = x2 - x1;
	double deltaY = y2 - y1;

	double deltaXSquare = deltaX * deltaX;
	double deltaYSquare = deltaY * deltaY;

	return sqrt(deltaXSquare + deltaYSquare);
}
