class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        unordered_map<char, int> last_pos;
        int max_pos = -1;
        int res = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s[i];
            auto c_pos = last_pos.find(c);
            if (c_pos != last_pos.end()) max_pos = max(max_pos, c_pos->second);
            res = max(res, i - max_pos);
            last_pos[c] = i;
        }
        return res;
    }
};
