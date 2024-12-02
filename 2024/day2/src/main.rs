use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file!");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_reports(input: &str) -> Vec<Vec<u32>> {
    let mut reports = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let mut report = Vec::new();

        for word in line.split_whitespace() {
            report.push(word.parse().expect("Invalid reading in input file!"));
        }

        reports.push(report);
    }

    reports
}

fn is_safe_report(report: &Vec<u32>) -> bool {
    let potentially_valid = report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b);

    if potentially_valid {
        for i in 0..report.len() - 1 {
            if u32::abs_diff(report[i], report[i + 1]) > 3 {
                return false;
            }
        }

        return true;
    }

    false
}

fn part_one(input: &str) -> u32 {
    let mut result = 0;

    let reports = get_reports(input);

    for report in &reports {
        if is_safe_report(report) {
            result += 1;
        }
    }

    result
}

fn print_report(report: &Vec<u32>) {
    print!("Report: ");

    for reading in report {
        print!("{reading} ");
    }

    println!();
}

fn part_two(input: &str) -> u32 {
    let mut result = 0;

    let reports = get_reports(input);

    for report in &reports {
        if is_safe_report(report) {
            result += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);

            if is_safe_report(&report) {
                // print_report(&report);
                result += 1;
                break;
            }
        }
    }

    result
}
