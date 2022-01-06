#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <unordered_map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    set<string> m;
    int n;
    char s[31];
    int res = 0;
    gets(s);
    sscanf(s, "%d", &n);
    for (int i = 0; i < n; i++) {
        gets(s);
        if (m.find(s) != m.end()) res++;
        else m.insert(s);
    }
    cout << res << '\n';
    return 0;
}
