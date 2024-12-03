reports: list[list[int]] = []

with open("./puzzle_input.txt", "r") as file_input:
    line = file_input.readline().strip()
    while line is None or line != "":
        reports.append([int(i) for i in line.split(" ")])
        line = file_input.readline().strip()

# reports = [
#     [7, 6, 4, 2, 1],
#     [1, 2, 7, 8, 9],
#     [9, 7, 6, 2, 1],
#     [1, 3, 2, 4, 5],
#     [8, 6, 4, 4, 1],
#     [1, 3, 6, 7, 9],
# ]


def report_is_unidirectional(rep: list[int]):
    is_ascending = rep[0] < rep[1]

    is_unidirectional = True
    for i in range(0, len(rep) - 1):
        if is_ascending:
            if rep[i] > rep[i + 1]:
                is_unidirectional = False
                break

        else:
            if rep[i + 1] > rep[i]:
                is_unidirectional = False
                break

    return is_unidirectional


def steps_are_within_range(steps: list[int]):
    for s in steps:
        if s not in [1, 2, 3]:
            return False

    return True


def report_is_safe(report: list[int]):
    if not report_is_unidirectional(report):
        return False

    steps: list[int] = []
    for i in range(0, len(report) - 1):
        steps.append(abs(report[i] - report[i + 1]))
        if len(steps) == 0:
            continue

        if not steps_are_within_range(steps):
            return False

    return True


def try_report_correction(report: list[int]):
    for i in range(len(report)):
        corrected = report.copy()
        corrected.pop(i)

        if report_is_safe(corrected):
            return corrected

    return None


def get_safe_reports_amount(report_list: list[list[int]], with_dampener=False) -> int:
    safe = 0

    for report in report_list:
        if report_is_safe(report):
            safe += 1
        else:
            if not with_dampener:
                continue

            corrected_report = try_report_correction(report)
            if corrected_report is not None:
                safe += 1

    return safe


safe_reports = get_safe_reports_amount(reports.copy())
print("Safe reports:", safe_reports)

dampener_safe_reports = get_safe_reports_amount(reports.copy(), with_dampener=True)
print("(Dampened) Safe reports:", dampener_safe_reports)
