struct Number {
    int val;
    int pos;
};

bool CmpNumbersByValue(const Number& a, const Number& b) {
    return a.val <  b.val;
}

const Number& FindNumberByValue(const vector<Number>& sorted_nums, int value) {
    int l = 0, r = sorted_nums.size() - 1;
    while (l < r) {
        int c = (l + r) / 2;
        if (sorted_nums[c].val < value) l = c + 1; else r = c;
    }
    return sorted_nums[l];
}

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        int n = nums.size();

        // Sort array of nums
        vector<Number> sorted_nums;
        for (int i = 0; i < n; i++) sorted_nums.push_back(Number{nums[i], i});
        sort(sorted_nums.begin(), sorted_nums.end(), &CmpNumbersByValue);

        // Loop through the sorted nums and use bin-search to find a pair
        for (int i = 0; i < n; i++) {
            auto a = sorted_nums[i];
            auto b = FindNumberByValue(sorted_nums, target - a.val);

            // A hack to avoid using the same number twice.
            // We will still find the solution later!
            if (a.pos == b.pos) continue;

            if (a.val + b.val == target) return vector<int>{a.pos, b.pos};
        }

        throw "Solution not exist!";
    }
};
