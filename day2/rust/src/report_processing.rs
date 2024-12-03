fn report_is_unidirectional(report: &Vec<i32>) -> bool {
    let is_ascending = report[0] < report[1];

    for i in 0..report.len() - 1 {
        if is_ascending {
            if report[i] > report[i + 1] {
                return false;
            }
        } else {
            if report[i + 1] > report[i] {
                return false;
            }
        }
    }

    return true;
}

fn steps_are_within_range(steps: &Vec<i32>) -> bool {
    for s in steps {
        if !([1, 2, 3].contains(s)) {
            return false;
        }
    }

    return true;
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    if !report_is_unidirectional(&report) {
        return false;
    }

    let mut steps: Vec<i32> = Vec::new();
    for i in 0..report.len() - 1 {
        steps.push((report[i] - report[i + 1]).abs());
        if steps.len() == 0 {
            continue;
        }

        if !steps_are_within_range(&steps) {
            return false;
        }
    }

    return true;
}

fn try_report_correction(report: &Vec<i32>) -> Option<Vec<i32>> {
    for i in 0..report.len() {
        let mut corrected = report.clone();
        corrected.remove(i);

        if report_is_safe(&corrected) {
            return Some(corrected);
        }
    }

    return None;
}

pub fn get_safe_reports_amount(report_list: &Vec<Vec<i32>>, with_dampener: Option<bool>) -> i32 {
    let mut safe_reports = 0;
    let use_dampener = with_dampener.unwrap_or(false);

    for report in report_list {
        if report_is_safe(&report) {
            safe_reports += 1;
        } else {
            if !use_dampener {
                continue;
            }

            let corrected_report: Option<Vec<i32>> = try_report_correction(report);
            if corrected_report.is_some() {
                safe_reports += 1;
            }
        }
    }

    return safe_reports;
}
