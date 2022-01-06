#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    int n;
    vector<char> m[26];
    cin >> n;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        m[s[0] - 'a'].push_back(s[1]);
    }
    string s;
    cin >> s;
    for (char c : m[s[0] - 'a']) cout << s << c << '\n';
    return 0;
}
