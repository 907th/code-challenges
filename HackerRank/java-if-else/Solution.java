import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);
	int n = in.nextInt();
	String s = "";
	if (n % 2 == 1) s = "Weird";
	if (n % 2 == 0 && (2 <= n && n <= 5)) s = "Not Weird";
	if (n % 2 == 0 && (6 <= n && n <= 20)) s = "Weird";
	if (n % 2 == 0 && n > 20) s = "Not Weird";
    System.out.println(s);
  }
}
