fn main() {
    let a: i32 = 0;
    let b: i32 = 0;
    let operator: char = '+';
    match operator {
        '+' => print!("{}", a + b),
        '-' => print!("{}", a - b),
        '/' => {
            if b != 0 {
                print!("{}", a / b);
            } else {
                print!("Division by 0 is undefined");
            }
        }
        '*' => print!("{}", a * b),
        '%' => {
            if b != 0 {
                print!("{}", a % b);
            } else {
                print!("Mod 0 is undefined");
            }
        }
        _ => print!("invalid operator"),
    };
}
