use microcrab::Value;

fn main() {
    let a = Value::from(-4.0);
    let b = Value::from(2.0);
    let c = 2.0 + (a + b);

    let n = Value::from(3.0) + 2.0;
    let m = 3.0 + Value::from(2.0);
    println!("C is {}", c);
    println!("n: {}, m: {}", n, m);
}
