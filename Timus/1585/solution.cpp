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
    int cnt[3];
    cnt[0] = cnt[1] = cnt[2] = 0;
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        string a, b;
        cin >> a >> b;
        if (a.compare("Emperor") == 0) cnt[0]++;
        if (a.compare("Little") == 0) cnt[1]++;
        if (a.compare("Macaroni") == 0) cnt[2]++;
    }
    int max = 0;
    if (cnt[1] > cnt[max]) max = 1;
    if (cnt[2] > cnt[max]) max = 2;
    if (max == 0) cout << "Emperor";
    if (max == 1) cout << "Little";
    if (max == 2) cout << "Macaroni";
    cout << " Penguin\n";
    return 0;
}
