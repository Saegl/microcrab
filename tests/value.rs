use microcrab::Value;

#[test]
fn expression() {
    let a = 2.5 * (5.0 + 2.0) + Value::from(0.0);
    assert!(a.close_to(17.5));
}
