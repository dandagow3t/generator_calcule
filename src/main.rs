use rand::Rng;
use std::collections::HashSet;

fn generate_subunit_sums_and_subtractions(with_nine: u32, to_nine: u32, total: u32, range: std::ops::RangeInclusive<u32>) {
    let mut rng = rand::thread_rng();
    let mut count = 0;
    let mut with_nine_remaining = with_nine;
    let mut to_nine_remaining = to_nine;
    let mut used_problems = HashSet::new();
    
    while count < total {
        // Decide which type of problem to generate
        let problem_type = if with_nine_remaining > 0 && to_nine_remaining > 0 {
            rng.gen_range(0..=2)
        } else if with_nine_remaining > 0 {
            if rng.gen_bool(0.3) { 0 } else { 2 }
        } else if to_nine_remaining > 0 {
            if rng.gen_bool(0.3) { 1 } else { 2 }
        } else {
            2
        };

        match problem_type {
            0 if with_nine_remaining > 0 => {
                // Subtraction with 9
                let a = rng.gen_range(11..=18);
                let problem = (a, 9, '-');
                if !used_problems.contains(&problem) {
                    println!("{} - {} =", a, 9);
                    used_problems.insert(problem);
                    with_nine_remaining -= 1;
                    count += 1;
                }
            },
            1 if to_nine_remaining > 0 => {
                // Subtraction to 9
                let a = rng.gen_range(11..=18);
                let b = a - 9;
                let problem = (a, b, '-');
                if !used_problems.contains(&problem) {
                    println!("{} - {} =", a, b);
                    used_problems.insert(problem);
                    to_nine_remaining -= 1;
                    count += 1;
                }
            },
            _ => {
                // Regular addition or subtraction
                let a = rng.gen_range(range.clone());
                let b = rng.gen_range(range.clone());
                
                // Skip if numbers are equal, consecutive, or involve 1
                if a == b || a == b + 1 || a == b - 1 || a == 1 || b == 1 {
                    continue;
                }
                
                let operation = if rng.gen_bool(0.5) { '+' } else { '-' };
                
                match operation {
                    '+' => {
                        let problem = (a, b, '+');
                        let reverse_problem = (b, a, '+');
                        if !used_problems.contains(&problem) && !used_problems.contains(&reverse_problem) {
                            println!("{} + {} =", a, b);
                            used_problems.insert(problem);
                            count += 1;
                        }
                    },
                    '-' => {
                        if a >= b {
                            let problem = (a, b, '-');
                            if !used_problems.contains(&problem) {
                                println!("{} - {} =", a, b);
                                used_problems.insert(problem);
                                count += 1;
                            }
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }
}

fn main() {
    // Example: 5 problems with subtraction with 9, 5 problems with subtraction to 9,
    // and 20 other random problems
    generate_subunit_sums_and_subtractions(10, 10, 50, 1..=19);
    
    println!("Done");
}
