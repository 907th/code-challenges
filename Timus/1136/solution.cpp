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

typedef vector<int>::iterator VIIT;

struct Tree {
    int id;
    Tree *left, *right;
    Tree(int _id) : id{_id} {};
};

Tree* make_tree(VIIT begin, VIIT end) {
    if (begin == end) return NULL;
    end--;
    Tree *t = new Tree(*end);
    VIIT split = begin;
    while (split != end && *split < t->id) split++;
    t->left = make_tree(begin, split);
    t->right = make_tree(split, end);
    // cout << "tree root " << t->id << " left " << (t->left ? t->left->id : -1) << " right " << (t->right ? t->right->id : -1) << '\n';
    return t;
}

void even_order(const Tree *t, vector<int> &res) {
    if (t == NULL) return;
    even_order(t->right, res);
    even_order(t->left, res);
    res.push_back(t->id);
}

int main() {
    int n;
    cin >> n;
    vector<int> odd;
    for (int i = 0; i < n; i++) {
        int id;
        cin >> id;
        odd.push_back(id);
    }
    Tree *t = make_tree(odd.begin(), odd.end());
    vector<int> res;
    even_order(t, res);
    for (int id : res) cout << id << '\n';
    return 0;
}
