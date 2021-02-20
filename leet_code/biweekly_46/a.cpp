#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t

class Solution {
public:
    bool is_nice(string s){
        set<char> chars;
        for(char c : s) chars.insert(c);

        for(char c : chars) if(!(chars.count(tolower(c)) && chars.count(toupper(c)))) return false;
        return true;
    }

    string longestNiceSubstring(string s) {
        string max = "";
        for(int i = 0; i < s.length(); i++){
            for(int j = i + 1; j <= s.length(); j++){
                string sub = s.substr(i, j - i);
                if(is_nice(sub) && sub.length() > max.length()) max = sub;
            }
        }
        return max;
    }
};
