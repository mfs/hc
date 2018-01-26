extern crate clap;
extern crate term;
extern crate rand;
extern crate libc;

use clap::{App, Arg};
use std::io::{Read, Write};
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
            "shl" => self.f2(|a, b| a.wrapping_shl(b as u32)),
            "shr" => self.f2(|a, b| a.wrapping_shr(b as u32)),
            "rol" => self.f2(|a, b| a.rotate_left(b as u32)),
            "ror" => self.f2(|a, b| a.rotate_right(b as u32)),
            "mod" => self.f2(|a, b| a % b),
            "gcd" => self.f2(gcd),
            "swp" => {
                if self.stack.len() > 1 {
                    self.stack.swap(0, 1);
                }
            },
            "rlu" => {
                if self.stack.len() > 1 {
                    let x = self.stack.remove(0);
                    self.stack.push(x);
                }
            },
            "rld" => {
                if self.stack.len() > 1 {
                    let x = self.stack.pop().unwrap();
                    self.stack.insert(0, x);
                }
            },
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

const HELP_TEXT: &str = "NUMBERS:
    All numbers are displayed and input in base 16.

OPERATORS:
    +        wrapping addition          (2 -- 1)
    -        wrapping subtraction       (2 -- 1)
    *        wrapping multiplication    (2 -- 1)
    /        division                   (2 -- 1)
    not      bitwise invert             (1 -- 1)
    and      and                        (2 -- 1)
    or       or                         (2 -- 1)
    xor      exclusive or               (2 -- 1)
    shl      shift left                 (2 -- 1)
    shr      shift right                (2 -- 1)
    rol      rotate left                (2 -- 1)
    ror      rotate right               (2 -- 1)
    mod      modulus                    (2 -- 1)
    swp      swap top two items         (2 -- 2)
    rlu      rotate stack up            (n -- n)
    rld      rotate stack down          (n -- n)
    clr      clear stack                (n --  )
    gcd      greatest common divosor    (2 -- 1)
    rand     random 64 bit number       (  -- 1)

    The parentheses indicate the number of operands
    popped off the stack followed by --, followed
    by the number of operands pushed onto the stack.
";

fn main() {
    let matches = App::new("hc")
        .version("0.1.0")
        .arg(
            Arg::with_name("EXPR")
            .help("Expression. e.g. '5 3 *'")
            .short("e")
            .takes_value(true)
        )
        .after_help(HELP_TEXT)
        .get_matches();

    if let Some(x) = matches.value_of("EXPR") {
        calculate(x);
    } else {
        if istty() {
            interactive();
        } else {
            let mut s = String::new();
            match std::io::stdin().read_to_string(&mut s) {
                Ok(_) => calculate(&s),
                Err(_) => eprintln!("error: non utf-8 input"),
            }
        }
    }
}

fn istty() -> bool {
    (unsafe { libc::isatty(libc::STDIN_FILENO as i32)} != 0)
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
