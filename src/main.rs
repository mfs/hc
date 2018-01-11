extern crate clap;
extern crate term;
extern crate rand;

use clap::{App, Arg};
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

    fn div(&mut self) {
        let y = self.pop();
        let x = self.pop();

        if y != 0 {
            self.push(x / y);
        } else {
            println!("division by zero");
            self.push(x);
            self.push(y);
        }
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

            let mut t = term::stdout().unwrap();
            t.reset().unwrap();
            write!(t, "{}: ", l - i - 1).unwrap();
            print_u64(*n);
        }
    }

    fn cmd(&mut self, cmd: &str) {
        match cmd {
            "+" => self.f2(|a, b| a.wrapping_add(b)),
            "-" => self.f2(|a, b| a.wrapping_sub(b)),
            "*" => self.f2(|a, b| a.wrapping_mul(b)),
            "/" => self.div(),
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

fn print_u64(n: u64) {
    let w = n.leading_zeros() / 4;

    let padding = "0".repeat(w as usize);

    let mut t = term::stdout().unwrap();

    t.attr(term::Attr::Dim).unwrap();
    t.fg(term::color::WHITE).unwrap();
    write!(t, "{}", padding).unwrap();

    t.reset().unwrap();
    if n > 0 {
        writeln!(t, "{:x}", n).unwrap();
    } else {
        writeln!(t).unwrap();
    }
}

fn main() {
    let matches = App::new("hc")
        .version("0.0.1")
        .arg(
            Arg::with_name("INPUT")
        )
        .get_matches();

    match matches.value_of("INPUT") {
        Some(x) => calculate(x),
        None => interactive(),
    }
}


fn calculate(cmds: &str) {
    let mut stack = Stack::new();

    for cmd in cmds.split(' ') {
        stack.cmd(cmd.trim());
    }

    stack.print();
}

fn interactive() {
    let mut stack = Stack::new();

    loop {
        stack.print();
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut cmds = String::new();

        std::io::stdin().read_line(&mut cmds).unwrap();

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
