#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct Item {
    int value;
    int weight;

    double value_per_unit() const {
        return (double) value / weight;
    }

    double partial_value(int w) const {
        return (double) w * value_per_unit();
    }
};

bool items_cmp_func(const Item &a, const Item &b) {
    return a.value_per_unit() > b.value_per_unit();
}

double get_optimal_value(int capacity, vector<Item> sorted_items) {
    double value = 0.0;
    for (const Item &item : sorted_items) {
        if (capacity == 0) break;
        int w = min(capacity, item.weight);
        value += item.partial_value(w);
        capacity -= w;
    }
    return value;
}

int main() {
    int n;
    int capacity;
    cin >> n >> capacity;
    vector<Item> items(n);
    for (int i = 0; i < n; i++) cin >> items[i].value >> items[i].weight;
    sort(items.begin(), items.end(), items_cmp_func);
    double optimal_value = get_optimal_value(capacity, items);
    cout.precision(10);
    cout << optimal_value << endl;
    return 0;
}
