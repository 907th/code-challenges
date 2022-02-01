#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

const char* FACULTIES[4] = {
    "Slytherin",
    "Hufflepuff",
    "Gryffindor",
    "Ravenclaw"
};

int main() {
    int n;
    char names[1000][201];
    int faculties[1000];

    char buf[201];
    gets(buf);
    sscanf(buf, "%d", &n);
    for (int i = 0; i < n; i++) {
        gets(names[i]);
        gets(buf);
        int x = -1;
        for (int j = 0; j < 4; j++) {
            if (strcmp(FACULTIES[j], buf) == 0) {
                x = j;
                break;
            }
        }
        assert(x != -1);
        faculties[i] = x;
    }

    for (int j = 0; j < 4; j++) {
        printf("%s:\n", FACULTIES[j]);
        for (int i = 0; i < n; i++)
            if (faculties[i] == j)
                printf("%s\n", names[i]);
        if (j < 3) printf("\n");
    }
    return 0;
}
