
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
        match self.stack.pop() {
            Some(x) => x,
            None => {
                eprintln!("error: stack underflow");
                std::process::exit(1);
            },
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    let cmds = "100 200 + 5 * 10 / 90 gcd";

    for cmd in cmds.split(' ') {
        match cmd {
            "+" => stack.f2(|a, b| a + b),
            "-" => stack.f2(|a, b| a - b),
            "*" => stack.f2(|a, b| a * b),
            "/" => stack.f2(|a, b| a / b),
            "not" => stack.f1(|a| !a),
            "and" => stack.f2(|a, b| a & b),
            "or" => stack.f2(|a, b| a | b),
            "xor" => stack.f2(|a, b| a ^ b),
            "mod" => stack.f2(|a, b| a % b),
            "gcd" => stack.f2(gcd),
            "swp" => stack.stack.swap(0, 1),
            "clr" => stack.stack.clear(),
            "rand" => stack.stack.push(42),
            _ => {
                let x = cmd.parse().unwrap();
                stack.push(x);
            },
        }
    }

    println!("{}", stack.pop());
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}
