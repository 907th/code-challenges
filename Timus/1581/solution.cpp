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

int main() {
    int n;
    cin >> n;
    int s = 0;
    int v[1000];
    int k[1000];
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        if (s == 0) {
            v[0] = x;
            k[0] = 1;
            s = 1;
        } else {
            if (x == v[s - 1]) {
                k[s - 1]++;
            } else {
                v[s] = x;
                k[s] = 1;
                s++;
            }
        }
    }
    for (int i = 0; i < s; i++) {
        cout << k[i] << ' ' << v[i];
        cout << (i < s - 1 ? ' ' : '\n');
    }
    return 0;
}
