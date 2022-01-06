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

// A  B C  D
string premium(char c) {
    if (c == 'A' || c == 'D') return "window";
    else return "aisle";
}

// A B  C D  E F
string business(char c) {
    if (c == 'A' || c == 'F') return "window";
    else return "aisle";

}

// A B C  D E F G  H J K
string ordinary(char c) {
    if (c == 'A' || c == 'K') return "window";
    else if (c == 'C' || c == 'D' || c == 'G' || c == 'H') return "aisle";
    else return "neither";

}

int main() {
    int n;
    char c;
    scanf("%d%c", &n, &c);
    if (n == 1 || n == 2) cout << premium(c);
    else if (n <= 20) cout << business(c);
    else cout << ordinary(c);
    cout << '\n';
    return 0;
}
