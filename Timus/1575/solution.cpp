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

const char *RED_LINE[] = {
    "7_klyuchey",
    "Sortirovochnaya",
    "China_town",
    "Zarechny",
    "City",
    "1905_year_square",
    "Kuybyshevskaya",
    "Sibirskaya",
    "Siniye_kamni",
    "Lechebnaya",
    "Varshavskaya",
    "Kompressornaya",
    "Koltsovo"
};

const char *LIGHT_BLUE_LINE[] = {
    "Pobedy",
    "I_Pyatiletki_square",
    "Zvezda",
    "Uralskaya",
    "Shevchenko",
    "Teatralnaya",
    "Kuybyshevskaya",
    "Oborony_square",
    "Mayakovskaya",
    "Parkovaya",
    "Botanicheskaya",
    "Samolyotnaya",
    "Vtorchermet",
    "Keramicheskaya"
};

const char *GREEN_LINE[] = {
    "Bakinskih_Komissarov",
    "Prospekt_Kosmonavtov",
    "Uralmash",
    "Mashinostroiteley",
    "Uralskaya",
    "Dinamo",
    "1905_year_square",
    "Geologicheskaya",
    "Bazhovskaya",
    "Chkalovskaya",
    "Botanicheskaya",
    "Shcherbakovskaya",
    "Uktusskie_Gory",
    "Nizhne_Isetskaya",
    "Himmash"
};

const char *VIOLET_LINE[] = {
    "Taganskaya",
    "Elmash",
    "Turbinnaya",
    "Pionerskaya",
    "Shevchenko",
    "1905_year_square",
    "Moskovskaya",
    "Central_stadium",
    "Kraulya",
    "Metallurgov",
    "MEGA"
};

const char *ORANGE_LINE[] = {
    "Vilonovskaya",
    "Gagarinskaya",
    "Vostochnaya",
    "Kuybyshevskaya",
    "Geologicheskaya",
    "Dvorets_sporta",
    "Aviatsionnaya",
    "Voennaya",
    "Sovhoznaya"
};

const char *DARK_BLUE_LINE[] = {
    "Kalinovskaya",
    "Italyanskaya",
    "Ozyornaya",
    "Shefskaya",
    "Komsomolskaya",
    "Gagarinskaya",
    "Teatralnaya",
    "Geologicheskaya",
    "Posadskaya",
    "Volgogradskaya",
    "Yugo_zapadnaya",
    "Akademicheskaya"
};

const char *YELLOW_LINE[] = {
    "University",
    "Kamennye_palatki",
    "Vtuzgorodok",
    "Vostochnaya",
    "Teatralnaya",
    "1905_year_square",
    "Kommunarov_square",
    "Verh_Isetskaya",
    "Tatishchevskaya",
    "Zelyony_ostrov"
};

const char *BROWN_LINE[] = {
    "Kommunarov_square",
    "City",
    "Uralskaya",
    "Pionerskaya",
    "Gagarinskaya",
    "Vtuzgorodok",
    "Sibirskaya",
    "Oborony_square",
    "Bazhovskaya",
    "Dvorets_sporta",
    "Posadskaya",
    "Moskovskaya",
    "Kommunarov_square"
};

const int N = 70; // Total stations count
map<string, int> numbers;
int edges[N][N];
int dist[N][N];

int register_station(string s) {
    auto it = numbers.find(s);
    if (it == numbers.end()) numbers[s] = numbers.size();
    return numbers[s];
}

void register_route(int a, int b) {
    edges[a][b] = 1;
    edges[b][a] = 1;
}

void register_line(const char **line, int n) {
    int last_num = -1;
    for (int i = 0; i < n; i++) {
        int cur_num = register_station(line[i]);
        if (last_num != -1) register_route(last_num, cur_num);
        last_num = cur_num;
    }
}

void calculate_all_distances() {
    const int INF = 1000;
    for (int a = 0; a < N; a++)
        for (int b = 0; b < N; b++)
            if (a != b) dist[a][b] = edges[a][b] == 1 ? 1 : INF;
            else dist[a][b] = 0;
    for (int c = 0; c < N; c++)
        for (int a = 0; a < N; a++)
            for (int b = 0; b < N; b++)
                if (dist[a][b] > dist[a][c] + dist[c][b])
                    dist[a][b] = dist[a][c] + dist[c][b];
}

void register_all_lines() {
    register_line(RED_LINE,        sizeof(RED_LINE) /        sizeof(char*));
    register_line(LIGHT_BLUE_LINE, sizeof(LIGHT_BLUE_LINE) / sizeof(char*));
    register_line(GREEN_LINE,      sizeof(GREEN_LINE) /      sizeof(char*));
    register_line(VIOLET_LINE,     sizeof(VIOLET_LINE) /     sizeof(char*));
    register_line(ORANGE_LINE,     sizeof(ORANGE_LINE) /     sizeof(char*));
    register_line(DARK_BLUE_LINE,  sizeof(DARK_BLUE_LINE) /  sizeof(char*));
    register_line(YELLOW_LINE,     sizeof(YELLOW_LINE) /     sizeof(char*));
    register_line(BROWN_LINE,      sizeof(BROWN_LINE) /      sizeof(char*));
}

void initialize() {
    memset(edges, 0, sizeof(edges));
    register_all_lines();
    calculate_all_distances();
}

int main() {
    initialize();

    int t;
    cin >> t;
    for (int i = 0; i < t; i++) {
        string s, f;
        cin >> s >> f;
        int a = numbers[s];
        int b = numbers[f];
        cout << dist[a][b] << '\n';
    }
    return 0;
}
