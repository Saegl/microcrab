use microcrab::Value;

fn main() {
    let a = Value::from(-4.0);
    let b = Value::from(2.0);
    let c = (a + b) * 2.0;
    println!("C is {}", c);
}
