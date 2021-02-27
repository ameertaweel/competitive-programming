/* Compile and run with:
g++ file.cpp -std=c++17 -O2 -Wall -o out && ./out < in
*/
#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t
#define ll long long

#define FAST_IO \
ios::sync_with_stdio(0);\
cin.tie(0);\
cout.tie(0);

struct Shop {
	intfl distance;
	intfl price;
	intfl stock;
};

int main(){
	FAST_IO;

	intfl n;
	cin >> n;

	vector<Shop> shops(n);

	for(auto& shop : shops){
		cin >> shop.distance >> shop.price >> shop.stock;
	}

	sort(shops.begin(), shops.end(), [](auto& shop1, auto& shop2){
		return shop1.price < shop2.price;
	});

	for(auto& shop : shops){
		if(shop.stock > shop.distance){
			cout << shop.price;
			return 0;
		}
	}

	cout << -1;

	return 0;
}
