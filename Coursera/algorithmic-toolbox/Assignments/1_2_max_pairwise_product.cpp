#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

long long MaxPairwiseProduct(const vector<int>& numbers) {
    int n = numbers.size();

    int pos1 = 0;
    for (int i = 1; i < n; i++) {
        if (numbers[i] > numbers[pos1]) pos1 = i;
    }

    int pos2 = (pos1 == 0 ? 1 : 0);
    for (int i = pos2 + 1; i < n; i++) {
        if (i != pos1 && numbers[i] > numbers[pos2]) pos2 = i;
    }

    return  (long long ) numbers[pos1] * numbers[pos2];
}

int main() {
    int n;
    cin >> n;
    vector<int> numbers(n);
    for (int i = 0; i < n; ++i) {
        cin >> numbers[i];
    }
    cout << MaxPairwiseProduct(numbers) << endl;
    return 0;
}

