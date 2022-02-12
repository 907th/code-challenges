#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <deque>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

int main() {
    map<string, int> device_counts;
    map<string, int> device_prices;

    for (int i = 0; i < 6; i++) {
        string friend_name;
        string device_name;
        int device_price;
        cin >> friend_name;
        cin >> device_name;
        cin >> device_price;

        auto k = device_counts.find(device_name);
        auto p = device_prices.find(device_name);
        if (k == device_counts.end()) {
            device_counts[device_name] = 1;
            device_prices[device_name] = device_price;
        } else {
            device_counts[device_name] = k->second + 1;
            device_prices[device_name] = min(p->second, device_price);
        }
    }

    string name;
    int count = 0;
    int price;
    for (auto i : device_counts) {
        string n = i.first;
        int k = i.second;
        int p = device_prices[i.first];
        if (k > count || (k == count && p < price)) {
            name = n;
            count = k;
            price = p;
        }
    }
    cout << name << '\n';

    return 0;
}
