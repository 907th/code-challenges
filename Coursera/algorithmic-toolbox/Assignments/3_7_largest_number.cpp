#include <algorithm>
#include <sstream>
#include <iostream>
#include <vector>
#include <string>

using namespace std;

bool str_cmp_func(const string &a, const string &b) {
    string ab = a + b;
    string ba = b + a;
    for (int i = 0; i < ab.length(); i++) {
        if (ab[i] == ba[i]) continue;
        return ab[i] > ba[i];
    }
    return false;
}

string largest_number(vector<string> a) {
    sort(a.begin(), a.end(), str_cmp_func);
    stringstream ret;
    for (int i = 0; i < a.size(); i++) {
        ret << a[i];
    }
    string result;
    ret >> result;
    return result;
}

int main() {
    int n;
    cin >> n;
    vector<string> a(n);
    for (int i = 0; i < a.size(); i++) cin >> a[i];
    cout << largest_number(a) << endl;
}
