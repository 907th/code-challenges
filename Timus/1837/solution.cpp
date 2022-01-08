#include <iostream>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    int nc = 0;
    map<string, int> name2id;
    vector<int> graph[300];

    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int team_ids[3];
        for (int j = 0; j < 3; j++) {
            string name;
            cin >> name;
            auto it = name2id.find(name);
            if (it == name2id.end()) {
                name2id[name] = nc;
                team_ids[j] = nc;
                nc++;
            } else team_ids[j] = it->second;
        }
        graph[team_ids[0]].push_back(team_ids[1]);
        graph[team_ids[0]].push_back(team_ids[2]);
        graph[team_ids[1]].push_back(team_ids[0]);
        graph[team_ids[1]].push_back(team_ids[2]);
        graph[team_ids[2]].push_back(team_ids[0]);
        graph[team_ids[2]].push_back(team_ids[1]);
    }

    int isenbaev_nums[300];
    memset(isenbaev_nums, -1, sizeof(isenbaev_nums));
    auto isenbaev_it = name2id.find("Isenbaev");
    if (isenbaev_it != name2id.end()) {
        int isenbaev_id = isenbaev_it->second;
        queue<int> qu;
        qu.push(isenbaev_id);
        isenbaev_nums[isenbaev_id] = 0;
        while (!qu.empty()) {
            int top_id = qu.front();
            int top_num = isenbaev_nums[top_id];
            qu.pop();
            for (int i = 0; i < graph[top_id].size(); i++) {
                int next_id = graph[top_id][i];
                if (isenbaev_nums[next_id] == -1) {
                    qu.push(next_id);
                    isenbaev_nums[next_id] = top_num + 1;
                }
            }
        }
    }

    for (auto x : name2id) {
        int num = isenbaev_nums[x.second];
        cout << x.first << ' ';
        if (num == -1) cout << "undefined";
        else cout << num;
        cout << '\n';
    }

    return 0;
}
