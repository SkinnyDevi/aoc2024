import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class Main {
	public static void main(String[] args) {
		File input = new File("puzzle_input.txt");
		List<Integer> leftList = new ArrayList<>();
		List<Integer> rightList = new ArrayList<>();
		getInput(input, leftList, rightList);

		int distance = calcTotalDistance(leftList, rightList);
		System.out.println(distance);

		int similarityScore = calcSimilarityScore(leftList, rightList);
		System.out.println(similarityScore);
	}

	private static final Map<Integer, Integer> SCORES = new HashMap<>();

	private static int askScores(int n, List<Integer> rList) {
		if (SCORES.containsKey(n))
			return SCORES.get(n);

		int score = (int) (n * rList.stream().filter(a -> a == n).count());
		SCORES.put(n, score);
		return score;
	}

	private static int calcSimilarityScore(List<Integer> leftList, List<Integer> rightList) {
		int similarityScore = 0;
		for (int value : leftList)
			similarityScore += askScores(value, rightList);

		return similarityScore;
	}

	public static void getInput(File input, List<Integer> l_list, List<Integer> r_list) {
		try (BufferedReader reader = new BufferedReader(new FileReader(input))) {
			while (true) {
				String line = reader.readLine();
				if (line == null)
					break;
				String[] split = line.trim().split(" ");
				l_list.add(Integer.parseInt(split[0]));
				r_list.add(Integer.parseInt(split[split.length - 1]));
			}

		} catch (IOException e) {
			throw new RuntimeException(e);
		}
	}

	public static int calcTotalDistance(List<Integer> l_list, List<Integer> r_list) {
		int totalDistance = 0;

		if (l_list.size() != r_list.size()) {
			throw new RuntimeException("List are different size");
		}

		List<Integer> lSorted = l_list.stream().sorted().collect(Collectors.toList());
		List<Integer> rSorted = r_list.stream().sorted().collect(Collectors.toList());

		for (int i = 0; i < lSorted.size(); i++) {
			int left = lSorted.get(i);
			int right = rSorted.get(i);

			totalDistance += Math.abs(left - right);
		}

		return totalDistance;
	}
}