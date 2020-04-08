import java.util.Scanner;

public class Solution {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        long n = scanner.nextLong();
        long m = scanner.nextLong();
        long a = scanner.nextLong();
        long res = ((n + a - 1) / a) * ((m + a - 1) / a);
        System.out.println(res);
    }
}