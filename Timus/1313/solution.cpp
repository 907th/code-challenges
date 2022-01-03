#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    int a[100][100];
    int n;
    cin >> n;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            cin >> a[i][j];
    cout << a[0][0];
    for (int s = 1; s < n; s++) {
        int i = s, j = 0;
        while (i >= 0 && j < n) {
            cout << ' ' << a[i][j];
            i--; j++;
        }
    }
    for (int s = 1; s < n; s++) {
        int i = n - 1, j = s;
        while (i >= 0 && j < n) {
            cout << ' ' << a[i][j];
            i--; j++;
        }
    }
    cout << '\n';
    return 0;
}
