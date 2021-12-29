#include <iostream>
#include <vector>
#include <vector>

using namespace std;

int compute_min_refills(int dist, int tank, const vector<int> &stops) {
    int answer = 0;
    int stop_id = -1, stop_dist = 0;
    while (stop_dist < dist) {
        // Find next fatherst reachable stop
        int next_id = stop_id;
        int next_dist = stop_dist;
        while (next_dist < dist) {
            int following_dist = next_id + 1 < stops.size() ? stops[next_id + 1] : dist;
            // cout << "Follwoing " << following_dist << endl;
            if (following_dist - stop_dist > tank) break;
            next_id += 1;
            next_dist = following_dist;
            // cout << "Next id " << next_id << endl;
            // cout << "Next dist " << next_dist << endl;
        }
        if (next_dist > stop_dist) {
            answer += 1;
            stop_id = next_id;
            stop_dist = next_dist;
            // cout << "Fuel at " << stop_id << endl;
        } else {
            return -1;
        }
    }
    return answer - 1;
}


int main() {
    int d = 0, m = 0, n = 0;
    cin >> d >> m >> n;
    vector<int> stops(n);
    for (int i = 0; i < n; i++) cin >> stops[i];
    cout << compute_min_refills(d, m, stops) << endl;
    return 0;
}
