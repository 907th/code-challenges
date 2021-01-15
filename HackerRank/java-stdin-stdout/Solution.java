import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);

    int i = in.nextInt();
	double d = in.nextDouble();
	in.nextLine();
	String s = in.nextLine();

    System.out.println("String: " + s);
    System.out.println("Double: " + Double.toString(d));
    System.out.println("Int: " + Integer.toString(i));
  }
}
