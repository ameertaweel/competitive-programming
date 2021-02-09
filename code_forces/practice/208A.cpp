#include <bits/c++config.h>
#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include <regex>

using namespace std;

int main(){
	string song;
	cin >> song;

	size_t wub;
	while((wub = song.find("WUB")) != string::npos){
		song.replace(wub, 3, wub > 0 ? " " : "");
	}

	// Remove spaces at the end
	cout << regex_replace(song, regex(R"((\s*)$)"), "");

	return 0;
}
