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

int main() {
    string cipher;
    cin >> cipher;
    vector<char> plain;
    plain.reserve(cipher.size());
    for (char c : cipher) {
        if (plain.size() == 0) {
            plain.push_back(c);
        } else {
            char t = plain.back();
            if (c == t) plain.pop_back();
            else plain.push_back(c);
        }
    }
    string res(plain.begin(), plain.end());
    cout << res << '\n';
    return 0;
}
