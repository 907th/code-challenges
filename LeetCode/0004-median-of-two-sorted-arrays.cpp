class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        int nums1_size = nums1.size();
        int nums2_size = nums2.size();
        if ((nums1_size + nums2_size) % 2 == 1) {
            int pos = (nums1_size + nums2_size) / 2;
            int res = findElementOnPos(nums1, nums2, pos);
            return (double) res;
        } else {
            int pos1 = (nums1_size + nums2_size) / 2 - 1;
            int pos2 = (nums1_size + nums2_size) / 2;
            int res1 = findElementOnPos(nums1, nums2, pos1);
            int res2 = findElementOnPos(nums1, nums2, pos2);
            return (double) (res1 + res2) / 2.0;
        }
    }

private:
    const int NOT_FOUND = 1 << 30;

    int findElementOnPos(const vector<int>& nums1, const vector<int>& nums2, int pos) {
        int res = findElementOnPosInFirstArary(nums1, nums2, pos);
        if (res != NOT_FOUND) return res;

        res = findElementOnPosInFirstArary(nums2, nums1, pos);
        if (res == NOT_FOUND) throw "Element must be found now!";
        return res;
    }

    int findElementOnPosInFirstArary(const vector<int>& nums1, const vector<int>& nums2, int pos) {
        int nums1_size = nums1.size();
        int nums2_size = nums2.size();

        if (nums1_size == 0) return NOT_FOUND;

        int l = 0;
        int r = nums1_size - 1;
        while (l < r) {
            int c = (l + r) / 2;
            int c_num = nums1[c];

            int greater_count = 0;
            greater_count += greaterElementsCount(nums1, c_num);
            greater_count += greaterElementsCount(nums2, c_num);
            int less_or_equal_count = nums1_size + nums2_size - greater_count;

            if (less_or_equal_count < pos + 1) l = c + 1; else r = c;
        }

        int l_num = nums1[l];

        int less_count = 0;
        less_count += lessElementsCount(nums1, l_num);
        less_count += lessElementsCount(nums2, l_num);

        int greater_count = 0;
        greater_count += greaterElementsCount(nums1, l_num);
        greater_count += greaterElementsCount(nums2, l_num);

        int less_or_equal_count = nums1_size + nums2_size - greater_count;

        //printf("Searching pos %d\n", pos);
        //printf("Found element %d on pos %d\n", l_num, l);
        //printf("Less: %d, less or equal: %d, greater: %d\n", less_count, less_or_equal_count, greater_count);

        return (less_count < pos + 1 && less_or_equal_count >= pos + 1) ? l_num : NOT_FOUND;
    }

    int lessElementsCount(const vector<int>& nums, int val) {
        int nums_size = nums.size();
        int l = 0;
        int r = nums_size;
        while (l < r) {
            int c = (l + r) / 2;
            if (nums[c] < val) l = c + 1; else r = c;
        }
        return l;
    }

    int greaterElementsCount(const vector<int>& nums, int val) {
        int nums_size = nums.size();
        int l = 0;
        int r = nums_size;
        while (l < r) {
            int c = (l + r) / 2;
            if (nums[c] <= val) l = c + 1; else r = c;
        }
        return nums_size - l;
    }
};
