extern crate term;
extern crate rand;

use std::io::Write;
use rand::Rng;

struct Stack {
    stack: Vec<u64>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack: vec![],
        }
    }

    fn f2<F>(&mut self, func: F) where F: Fn(u64, u64) -> u64 {
        let y = self.pop();
        let x = self.pop();

        self.push(func(x, y));
    }

    fn f1<F>(&mut self, func: F) where F: Fn(u64) -> u64 {
        let x = self.pop();

        self.push(func(x));
    }

    fn push(&mut self, x: u64) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> u64 {
        self.stack.pop().unwrap_or(0)
    }

    fn print(&self) {
        let l = self.stack.len();
        for (i, n) in self.stack.iter().enumerate() {
            let s = format!("{:x}", n);
            let padding = "0".repeat(16 - s.len());

            let mut t = term::stdout().unwrap();

            t.reset().unwrap();
            write!(t, "{}: ", l - i - 1).unwrap();

            t.attr(term::Attr::Dim).unwrap();
            t.fg(term::color::WHITE).unwrap();
            write!(t, "{}", padding).unwrap();

            t.reset().unwrap();
            writeln!(t, "{}", s).unwrap();
        }
    }

    fn cmd(&mut self, cmd: &str) {
        match cmd {
            "+" => self.f2(|a, b| a + b),
            "-" => self.f2(|a, b| a - b),
            "*" => self.f2(|a, b| a * b),
            "/" => self.f2(|a, b| a / b),
            "not" => self.f1(|a| !a),
            "and" => self.f2(|a, b| a & b),
            "or" => self.f2(|a, b| a | b),
            "xor" => self.f2(|a, b| a ^ b),
            "mod" => self.f2(|a, b| a % b),
            "gcd" => self.f2(gcd),
            "swp" => self.stack.swap(0, 1),
            "clr" => self.stack.clear(),
            "rand" => self.stack.push(rand::thread_rng().gen()),
            _ => {
                match u64::from_str_radix(cmd, 16) {
                    Ok(x) => self.push(x),
                    Err(_) => println!("error: invalid input"),
                }
            },
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    let stdin = std::io::stdin();

    loop {
        stack.print();
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut cmds = String::new();

        stdin.read_line(&mut cmds).unwrap();

        if cmds.trim() == "quit" {
            break;
        }

        for cmd in cmds.split(' ') {
            stack.cmd(cmd.trim());
        }

    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}
