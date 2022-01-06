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

const char *names[3][9] = {
    {
        "Alice", "Ariel", "Aurora",
        "Phil", "Peter", "Olaf",
        "Phoebus", "Ralph", "Robin"
    },
    {
        "Bambi", "Belle", "Bolt",
        "Mulan", "Mowgli", "Mickey",
        "Silver", "Simba", "Stitch"
    },
    {
        "Dumbo", "Genie", "Jiminy",
        "Kuzko", "Kida", "Kenai",
        "Tarzan", "Tiana", "Winnie"
    }
};

int detect_pos(const string &name) {
    for (int i = 0; i < 3; i++) {
        auto f = find(names[i], names[i] + 9, name);
        if (f != names[i] + 9) return i;
    }
    throw 1;
}

int main() {
    int n;
    cin >> n;
    int pos = 0, ans = 0;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        int new_pos = detect_pos(s);
        ans += abs(new_pos - pos);
        pos = new_pos;
    }
    cout << ans << '\n';
    return 0;
}
