#include <bits/stdc++.h>

using namespace std;

#define intfss int_fast8_t
#define intfs int_fast16_t
#define intf int_fast32_t
#define intfl int_fast64_t

class Solution {
public:
    bool canChoose(vector<vector<int>>& groups, vector<int>& nums) {
        intfl last_stop = -1;
        for(auto& group : groups){
            if(last_stop == nums.size() - 1) return false;
            bool found;
            for(intfl i = last_stop + 1; i < nums.size(); i++){
                intfl j = i;
                found = true;
                for(auto& x : group){
                    if(nums[j] != x) {found = false; break;}
                    j++;
                }
                if(found) {last_stop = j - 1; break;}
            }
            if(!found) return false;
        }
        return true;
    }
};
