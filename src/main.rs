use rand::Rng;
use std::collections::HashSet;

/// Represents a single arithmetic operation with two operands and an operator.
/// 
/// # Examples
/// ```
/// let addition = Operation { a: 5, b: 3, op: '+' };     // represents "5 + 3"
/// let subtraction = Operation { a: 12, b: 7, op: '-' }; // represents "12 - 7"
/// ```
#[derive(Debug)]
struct Operation {
    a: u32,
    b: u32,
    op: char,
}

/// A builder for creating and managing a collection of arithmetic operations.
/// Ensures uniqueness of operations and provides methods for manipulation and display.
/// 
/// # Examples
/// ```
/// let problems = OpsBuilder::new()
///     .add_ops(generate_ops(5, 1..=10))          // Add 5 random operations
///     .add_ops(generate_sub_with_nine(3))        // Add 3 subtractions with 9
///     .shuffle()                                 // Randomize the order
///     .print();                                  // Display the problems
/// ```
struct OpsBuilder {
    operations: Vec<Operation>,
    used: HashSet<(u32, u32, char)>,
}

impl OpsBuilder {
    /// Creates a new empty OpsBuilder.
    /// 
    /// # Examples
    /// ```
    /// let builder = OpsBuilder::new();
    /// ```
    fn new() -> Self {
        OpsBuilder {
            operations: Vec::new(),
            used: HashSet::new(),
        }
    }

    /// Adds new operations to the builder while maintaining uniqueness.
    /// For addition operations, also prevents reverse duplicates (e.g., 2+3 and 3+2).
    /// 
    /// # Examples
    /// ```
    /// let builder = OpsBuilder::new()
    ///     .add_ops(vec![
    ///         Operation { a: 5, b: 3, op: '+' },
    ///         Operation { a: 8, b: 4, op: '-' }
    ///     ]);
    /// ```
    /// 
    /// Note: Adding "3 + 5" after "5 + 3" will be ignored as they're considered duplicates.
    fn add_ops(mut self, mut new_ops: Vec<Operation>) -> Self {
        for op in new_ops.drain(..) {
            let key = (op.a, op.b, op.op);
            let reverse_key = (op.b, op.a, op.op);
            if !self.used.contains(&key) && (op.op == '-' || !self.used.contains(&reverse_key)) {
                self.used.insert(key);
                if op.op == '+' {
                    self.used.insert(reverse_key);
                }
                self.operations.push(op);
            }
        }
        self
    }

    /// Randomly shuffles the order of all operations.
    /// 
    /// # Examples
    /// ```
    /// let shuffled_problems = OpsBuilder::new()
    ///     .add_ops(generate_ops(10, 1..=10))
    ///     .shuffle();  // Randomizes the order of the 10 operations
    /// ```
    fn shuffle(mut self) -> Self {
        let mut rng = rand::thread_rng();
        for i in (1..self.operations.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.operations.swap(i, j);
        }
        self
    }

    /// Prints all operations with proper spacing alignment.
    /// Single-digit numbers are padded with a space for better visual alignment.
    /// 
    /// # Examples
    /// ```
    /// OpsBuilder::new()
    ///     .add_ops(vec![
    ///         Operation { a: 15, b: 7, op: '+' },
    ///         Operation { a: 8, b: 12, op: '-' }
    ///     ])
    ///     .print();
    /// 
    /// // Output will look like:
    /// // 15 +  7 =
    /// //  8 - 12 =
    /// ```
    fn print(self) -> Self {
        for op in &self.operations {
            let a_space = if op.a < 10 { " " } else { "" };
            let b_space = if op.b < 10 { " " } else { "" };
            println!("{}{} {} {}{} =", a_space, op.a, op.op, b_space, op.b);
        }
        self
    }
}

/// Generates a specified number of random addition and subtraction operations.
/// Excludes operations where numbers are equal, consecutive, or involve 1.
/// 
/// # Examples
/// ```
/// let ops = generate_ops(5, 2..=10);
/// // Might generate operations like:
/// // 7 + 4
/// // 9 - 3
/// // 5 + 8
/// // 10 - 6
/// // 4 + 7
/// ```
/// 
/// Note: Will not generate operations like:
/// - 5 + 5 (equal numbers)
/// - 6 + 5 or 5 + 6 (consecutive numbers)
/// - 1 + 4 or 4 + 1 (operations involving 1)
fn generate_ops(n: u32, range: std::ops::RangeInclusive<u32>) -> Vec<Operation> {
    let mut rng = rand::thread_rng();
    let mut ops = Vec::new();
    let mut used = HashSet::new();

    while ops.len() < n as usize {
        let a = rng.gen_range(range.clone());
        let b = rng.gen_range(range.clone());
        
        // Skip if numbers are equal, consecutive, or involve 1
        if a == b || a == b + 1 || a == b - 1 || a == 1 || b == 1 {
            continue;
        }
        
        let operation = if rng.gen_bool(0.5) { '+' } else { '-' };
        
        match operation {
            '+' => {
                let key = (a, b, '+');
                let reverse_key = (b, a, '+');
                if !used.contains(&key) && !used.contains(&reverse_key) {
                    used.insert(key);
                    used.insert(reverse_key);
                    ops.push(Operation { a, b, op: '+' });
                }
            },
            '-' => {
                if a >= b {
                    let key = (a, b, '-');
                    if !used.contains(&key) {
                        used.insert(key);
                        ops.push(Operation { a, b, op: '-' });
                    }
                }
            },
            _ => unreachable!()
        }
    }
    ops
}

/// Generates a specified number of subtraction operations with 9 as the second operand.
/// 
/// # Examples
/// ```
/// let ops = generate_sub_with_nine(4);
/// // Will generate operations like:
/// // 18 - 9
/// // 15 - 9
/// // 13 - 9
/// // 11 - 9
/// ```
fn generate_sub_with_nine(n: u32) -> Vec<Operation> {
    let mut rng = rand::thread_rng();
    let mut ops = Vec::new();
    let mut used = HashSet::new();

    while ops.len() < n as usize {
        let a = rng.gen_range(11..=18);
        let key = (a, 9, '-');
        if !used.contains(&key) {
            used.insert(key);
            ops.push(Operation { a, b: 9, op: '-' });
        }
    }
    ops
}

/// Generates a specified number of subtraction operations that result in 9.
/// 
/// # Examples
/// ```
/// let ops = generate_sub_to_nine(4);
/// // Will generate operations like:
/// // 18 - 9
/// // 17 - 8
/// // 16 - 7
/// // 15 - 6
/// ```
/// 
/// Note: All these operations result in 9 when solved.
fn generate_sub_to_nine(n: u32) -> Vec<Operation> {
    let mut rng = rand::thread_rng();
    let mut ops = Vec::new();
    let mut used = HashSet::new();

    while ops.len() < n as usize {
        let a = rng.gen_range(11..=18);
        let b = a - 9;
        let key = (a, b, '-');
        if !used.contains(&key) {
            used.insert(key);
            ops.push(Operation { a, b, op: '-' });
        }
    }
    ops
}

fn main() {
    // Example usage of the builder pattern to generate a mixed set of arithmetic problems
    OpsBuilder::new()
        .add_ops(generate_ops(20, 1..=19))        // 20 random operations
        .add_ops(generate_sub_with_nine(5))       // 5 subtractions with 9
        .add_ops(generate_sub_to_nine(5))         // 5 subtractions to 9
        .shuffle()                                // Randomize all operations
        .print();                                 // Display them
    
    println!("Done");
}
