#include <iostream>
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
#include <cassert>

using namespace std;

map<string, string> passwords;
map<string, bool> statuses;

void do_register(string login, string password) {
    auto pwd = passwords.find(login);
    if (pwd == passwords.end()) {
        passwords[login] = password;
        printf("success: new user added\n");
    } else {
        printf("fail: user already exists\n");
    }
}

void do_login(string login, string password) {
    auto pwd = passwords.find(login);
    if (pwd == passwords.end()) {
        printf("fail: no such user\n");
    } else if (password.compare(pwd->second) != 0) {
        printf("fail: incorrect password\n");
    } else if (statuses[login]) {
        printf("fail: already logged in\n");
    } else {
        statuses[login] = true;
        printf("success: user logged in\n");
    }
}

void do_logout(string login) {
    auto pwd = passwords.find(login);
    if (pwd == passwords.end()) {
        printf("fail: no such user\n");
    } else if (!statuses[login]) {
        printf("fail: already logged out\n");
    } else {
        statuses[login] = false;
        printf("success: user logged out\n");
    }
}

int main() {
    char s[200];
    gets(s);
    int n;
    sscanf(s, "%d", &n);
    for (int i = 0; i < n; i++) {
        gets(s);
        char operation[10];
        char login[31];
        char password[31];
        operation[0] = 0;
        login[0] = 0;
        password[0] = 0;
        sscanf(s, "%s %s %s", operation, login, password);
        if (strcmp(operation, "register") == 0) {
            assert(strlen(s) == strlen(operation) + strlen(login) + strlen(password) + 2);
            do_register(login, password);
        } else if (strcmp(operation, "login") == 0) {
            assert(strlen(s) == strlen(operation) + strlen(login) + strlen(password) + 2);
            do_login(login, password);
        } else if (strcmp(operation, "logout") == 0) {
            assert(strlen(s) == strlen(operation) + strlen(login) + 1);
            do_logout(login);
        } else throw 1;
    }
    return 0;
}
