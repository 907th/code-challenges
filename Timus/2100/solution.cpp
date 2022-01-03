#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

bool plus_one(const string &s) {
    return s.find("+one") != string::npos;
}

int main() {
    int k = 0;
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        k++;
        if (plus_one(s)) k++;
    }
    k += 2;
    if (k == 13) k++;
    cout << k * 100 << '\n';
    return 0;
}
