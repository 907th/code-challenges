#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>

using namespace std;

struct Num {
    static const int N = 55;

    int num[N];

    Num() {
        memset(num, 0, sizeof(num));
    }

    void scan() {
        string s;
        cin >> s;
        reverse(s.begin(), s.end());
        int k = 0;
        for (char c : s) num[k++] = c - '0';
    }
};

int operator%(const Num &a, int b) {
    Num res;
    int rem = 0;
    for (int i = Num::N - 1; i >= 0; i--) {
        rem = rem * 10 + a.num[i];
        res.num[i] = rem / b;
        rem = rem % b;
    }
    return rem;
}

int main() {
    Num n;
    n.scan();
    cout << (n % 7) << '\n';
    return 0;
}
