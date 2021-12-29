#include <iostream>
#include <vector>

using namespace std;

vector<int> optimal_summands(int n) {
    vector<int> summands;
    int sum = 0;
    for (int i = 1; i <= n; i++) {
        if (n - sum >= i) {
            summands.push_back(i);
            sum += i;
        } else {
            summands[summands.size() - 1] += n - sum;
            break;
        }
    }
    return summands;
}

int main() {
    int n;
    cin >> n;
    vector<int> summands = optimal_summands(n);
    cout << summands.size() << endl;
    for (int i = 0; i < summands.size(); ++i) {
        cout << summands[i];
        cout << (i < summands.size() - 1 ? " " : "\n");
    }
}
