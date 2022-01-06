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
    int x = 1;
    int m[100][100];
    for (int s = n - 1; s >= 0; s--) {
        int r = 0, c = s;
        while (r < n && c < n) {
            m[r][c] = x++;
            r++;
            c++;
        }
    }
    for (int s = 1; s < n; s++) {
        int r = s, c = 0;
        while (r < n && c < n) {
            m[r][c] = x++;
            r++;
            c++;
        }
    }
    for (int r = 0; r < n; r++)
        for (int c = 0; c < n; c++) {
            cout << m[r][c];
            cout << (c < n - 1 ? ' ' : '\n');
        }
    return 0;
}
