import java.io.*;
import java.util.*;

class ExampleSolver {
  String number;
  String[] words;
  List<Integer>[] graph;

  static String[] numToChars = {
    "oqueuez", // 0
    "ij", // 1
    "abc", // 2
    "def", // 3
    "gh", // 4
    "kl", // 5
    "mn", // 6
    "prs", // 7
    "tuv", // 8
    "wxy" // 9
  };

  ExampleSolver(String number, String[] words) {
    this.number = number;
    this.words = words;
    graph = new List[number.length()];
  }

  boolean isMatching(int startPos, String word) {
    if (startPos + word.length() > number.length()) return false;
    for (int i = 0; i < word.length(); i++) {
      char nc = number.charAt(startPos + i);
      char wc = word.charAt(i);
      if (numToChars[nc - '0'].indexOf(wc) == -1) return false;
    }
    return true;
  }

  List<Integer> matchingWords(int startPos) {
    if (graph[startPos] != null) return graph[startPos];
    List<Integer> res = new ArrayList<>();
    for (int i = 0; i < words.length; i++) {
      String word = words[i];
      if (isMatching(startPos, word)) res.add(i);
    }
    graph[startPos] = res;
    return res;
  }

  List<String> solve() {
    Queue<Integer> queue = new LinkedList<>();
    int[] prev = new int[number.length() + 1];
    Arrays.fill(prev, -1);
    queue.add(0);
    while (queue.size() > 0) {
      int curPos = queue.remove();

      if (curPos == number.length()) {
        List<String> res = new ArrayList<>();
        int i = curPos;
        while (prev[i] != -1) {
          String word = words[prev[i]];
          res.add(0, word);
          i -= word.length();
        }
        return res;
      }

      for (int wordNum : matchingWords(curPos)) {
        String word = words[wordNum];
        int nextPos = curPos + word.length();
        if (prev[nextPos] != -1) continue;
        prev[nextPos] = wordNum;
        queue.add(nextPos);
      }
    }
    return null;
  }
}

public class Solution {
  Scanner in = new Scanner(System.in);
  PrintStream out = System.out;

  boolean solveExample() {
    String number = in.nextLine();
    if (number.equals("-1")) return false;

    int wordsCount = Integer.parseInt(in.nextLine());
    String[] words = new String[wordsCount];
    for (int i = 0; i < wordsCount; i++) {
      String word = in.nextLine();
      words[i] = word;
    }

    ExampleSolver solver = new ExampleSolver(number, words);
    List<String> result = solver.solve();
    if (result == null)
      out.println("No solution.");
    else
      out.println(String.join(" ", result));

    return true;
  }

  public static void main(String[] args) {
    Solution sol = new Solution();
    while (sol.solveExample());
  }
}
