#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <deque>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

deque<pair<int, int>> q;

void insert(int v, int i, int l) {
    while (q.size() > 0 && q.front().first < l) q.pop_front();
    while (q.size() > 0 && q.back().second < v) q.pop_back();
    q.push_back(make_pair(i, v));
}

int front() {
    return q.front().second;
}

int main() {
    int m;
    cin >> m;
    for (int i = 0; i < m; i++) {
        int v;
        cin >> v;
        insert(v, i, 0);
    }
    for (int i = m; true; i++) {
        cout << front() << '\n';
        int v;
        cin >> v;
        if (v == -1) break;
        else insert(v, i, i - m + 1);
    }
    return 0;
}
