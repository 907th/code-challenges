#include <iostream>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

string a(int n) {
    string ret = "sin(" + to_string(n) + ")";
    for (int i = n - 1; i >= 1; i--) {
        char sign = (i % 2 == 1 ? '-' : '+');
        ret = "sin(" + to_string(i) + sign + ret + ")";
    }
    return ret;
}

string s(int n) {
    string ret = "";
    for (int i = 1; i <= n; i++) {
        string exp = a(i) + "+" + to_string(n - i + 1);
        if (i == 1) ret = exp;
        else ret = "(" + ret + ")" + exp;
    }
    return ret;
}

int main() {
    int n = 0;
    cin >> n;
    cout << s(n) << '\n';
    return 0;
}
