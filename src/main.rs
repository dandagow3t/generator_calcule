use rand::Rng;
use std::collections::HashSet;

#[derive(Debug)]
struct Operation {
    a: u32,
    b: u32,
    op: char,
}

struct OpsBuilder {
    operations: Vec<Operation>,
    used: HashSet<(u32, u32, char)>,
}

impl OpsBuilder {
    fn new() -> Self {
        OpsBuilder {
            operations: Vec::new(),
            used: HashSet::new(),
        }
    }

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

    fn shuffle(mut self) -> Self {
        let mut rng = rand::thread_rng();
        for i in (1..self.operations.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.operations.swap(i, j);
        }
        self
    }

    fn print(self) -> Self {
        for op in &self.operations {
            let a_space = if op.a < 10 { " " } else { "" };
            let b_space = if op.b < 10 { " " } else { "" };
            println!("{}{} {} {}{} =", a_space, op.a, op.op, b_space, op.b);
        }
        self
    }
}

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
    OpsBuilder::new()
        .add_ops(generate_ops(20, 1..=19))
        .add_ops(generate_sub_with_nine(5))
        .add_ops(generate_sub_to_nine(5))
        .shuffle()
        .print();
    
    println!("Done");
}
