import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class Main {
	public static void main(String[] args) {
		File fileInput = new File("./puzzle_input.txt");
		List<List<Integer>> reports = parseInput(fileInput);

		int safeReports = getSafeReportsAmount(reports);
		int dampenedSafeReports = getSafeReportsAmount(reports, true);
		System.out.println("Safe reports: " + safeReports);
		System.out.println("(Dampened) Safe reports: " + dampenedSafeReports);
	}

	private static int getSafeReportsAmount(List<List<Integer>> reports) {
		return getSafeReportsAmount(reports, false);
	}

	private static int getSafeReportsAmount(List<List<Integer>> reports, boolean withDampener) {
		int safeReports = 0;

		for (List<Integer> report : reports) {
			if (reportIsSafe(report)) {
				safeReports++;
			} else {
				if (!withDampener)
					continue;

				Optional<List<Integer>> correctedReport = tryReportCorrection(report);
				if (correctedReport.isPresent())
					safeReports++;
			}
		}

		return safeReports;
	}

	private static Optional<List<Integer>> tryReportCorrection(List<Integer> report) {
		for (int i = 0; i < report.size(); i++) {
			List<Integer> corrected = new ArrayList<>(report);
			corrected.remove(i);

			if (reportIsSafe(corrected))
				return Optional.of(corrected);
		}

		return Optional.empty();
	}

	private static boolean reportIsSafe(List<Integer> report) {
		if (!reportIsUnidirectional(report))
			return false;

		List<Integer> steps = new ArrayList<>();
		for (int i = 0; i < report.size() - 1; i++) {
			steps.add(Math.abs(report.get(i) - report.get(i + 1)));
			if (i == 0)
				continue;

			if (!isWithinRange(steps))
				return false;
		}

		return true;
	}

	private static boolean isWithinRange(List<Integer> steps) {
		final int[] allowedSteps = new int[] { 1, 2, 3 };
		return steps.stream().allMatch(s -> Arrays.stream(allowedSteps).anyMatch(v -> v == s));
	}

	private static boolean reportIsUnidirectional(List<Integer> report) {
		boolean isAscending = report.get(0) < report.get(1);

		for (int i = 0; i < report.size() - 1; i++) {
			if (isAscending) {
				if (report.get(i) > report.get(i + 1))
					return false;
			} else {
				if (report.get(i + 1) > report.get(i))
					return false;
			}
		}

		return true;
	}

	static List<List<Integer>> parseInput(File input) {
		try (BufferedReader reader = new BufferedReader(new FileReader(input))) {
			List<List<Integer>> reports = new ArrayList<>();
			while (true) {
				String line = reader.readLine();
				if (line == null)
					break;
				reports.add(parseReportLevels(line));
			}

			return reports;
		} catch (IOException e) {
			throw new RuntimeException(e);
		}
	}

	private static List<Integer> parseReportLevels(String line) {
		String[] levels = line.split(" ");
		return Arrays.stream(levels).map(Integer::parseInt).collect(Collectors.toList());
	}
}