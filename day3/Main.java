package day3;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
	static class Pair<T, E> {
		private final T first;
		private final E last;

		public Pair(T first, E last) {
			this.first = first;
			this.last = last;
		}

		public T getFirst() {
			return first;
		}

		public E getLast() {
			return last;
		}
	}

	public static void main(String[] args) {
		File fileInput = new File("./puzzle_input.txt");
		final String MEMORY = getMemoryFromFile(fileInput);

		int multiplyOutput = multiplyMemory(MEMORY);
		int conditionalMultiplyOutput = multiplyMemory(MEMORY, true);

		System.out.println("Corrected calculation: " + multiplyOutput);
		System.out.println("(Conditional) Corrected calculation: " + conditionalMultiplyOutput);
	}

	private static int multiplyMemory(String memory) {
		return multiplyMemory(memory, false);
	}

	private static int multiplyMemory(String memory, boolean useConditionalMuls) {
		List<Pair<Integer, Integer>> foundMuls;
		if (useConditionalMuls)
			foundMuls = getValidConditionalMulInstances(memory);
		else
			foundMuls = getValidMulInstances(memory);

		int result = 0;
		for (Pair<Integer, Integer> mul : foundMuls)
			result += mul.first * mul.last;

		return result;
	}

	private static List<Pair<Integer, Integer>> getValidConditionalMulInstances(String memory) {
		final Pattern PATTERN = Pattern.compile("(do\\(\\)|don't\\(\\))|mul\\([0-9]{1,3},[0-9]{1,3}\\)");

		List<Pair<Integer, Integer>> foundMuls = new ArrayList<>();
		Matcher matcher = PATTERN.matcher(memory);
		boolean canParseMul = true;
		while (matcher.find()) {
			String foundMul = matcher.group();
			if (foundMul.equals("do()") || foundMul.equals("don't()")) {
				canParseMul = foundMul.equals("do()");
				continue;
			}

			if (!canParseMul)
				continue;

			foundMuls.add(mulStrToPair(foundMul.replace("mul(", "").replace(")", "")));
		}

		return foundMuls;
	}

	private static List<Pair<Integer, Integer>> getValidMulInstances(String memory) {
		final Pattern PATTERN = Pattern.compile("mul\\([0-9]{1,3},[0-9]{1,3}\\)");

		List<Pair<Integer, Integer>> foundMuls = new ArrayList<>();
		Matcher matcher = PATTERN.matcher(memory);
		while (matcher.find()) {
			String foundMul = matcher.group();
			foundMuls.add(mulStrToPair(foundMul.replace("mul(", "").replace(")", "")));
		}

		return foundMuls;
	}

	private static Pair<Integer, Integer> mulStrToPair(String mulStr) {
		String[] mulParamsStr = mulStr.split(",");
		int[] params = Arrays.stream(mulParamsStr).mapToInt(Integer::parseInt).toArray();

		return new Pair<Integer, Integer>(params[0], params[1]);
	}

	private static String getMemoryFromFile(File fileInput) {
		try (BufferedReader reader = new BufferedReader(new FileReader(fileInput))) {
			String content = "";
			while (true) {
				String line = reader.readLine();
				if (line == null)
					break;
				content += line;
			}

			return content;
		} catch (IOException e) {
			throw new RuntimeException(e);
		}
	}
}