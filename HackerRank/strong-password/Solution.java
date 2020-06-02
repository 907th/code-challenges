import java.io.*;
import java.util.*;

public class Solution {
  public static void main(String[] args) {
    Scanner in = new Scanner(System.in);

    String length = in.nextLine();
    String password = in.nextLine().trim();

    String numbers = "0123456789";
    String lower_case = "abcdefghijklmnopqrstuvwxyz";
    String upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    String special_characters = "!@#$%^&*()-+";

    boolean is_digit = password.chars().anyMatch(c -> numbers.indexOf(c) != -1);
    boolean is_lower = password.chars().anyMatch(c -> lower_case.indexOf(c) != -1);
    boolean is_upper = password.chars().anyMatch(c -> upper_case.indexOf(c) != -1);
    boolean is_special = password.chars().anyMatch(c -> special_characters.indexOf(c) != -1);

    int missing_chars = 0;
    if (!is_digit) missing_chars++;
    if (!is_lower) missing_chars++;
    if (!is_upper) missing_chars++;
    if (!is_special) missing_chars++;

    int missing_length = Math.max(0, 6 - password.length());

    System.out.println(Math.max(missing_length, missing_chars));
  }
}
