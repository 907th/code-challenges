#include <algorithm>
#include <iostream>
#include <climits>
#include <vector>

using namespace std;

struct Segment {
  int start, end;
};

bool cmp_segments(const Segment &a, const Segment &b) {
    return a.end < b.end;
}

vector<int> optimal_points(vector<Segment> segments) {
  vector<int> points;
  sort(segments.begin(), segments.end(), cmp_segments);
  int last = -1;
  for (int i = 0; i < segments.size(); i++) {
      if (segments[i].start > last) {
          last = segments[i].end;
          points.push_back(last);
      }
  }
  return points;
}

int main() {
  int n;
  cin >> n;
  vector<Segment> segments(n);
  for (int i = 0; i < n; ++i)
    cin >> segments[i].start >> segments[i].end;
  vector<int> points = optimal_points(segments);
  cout << points.size() << endl;
  for (int i = 0; i < points.size(); ++i) {
    cout << points[i];
    cout << (i < points.size() - 1 ? " " : "\n");
  }
}
