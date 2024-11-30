fn main() {
    let x: i32 = 1;
    let y: f64 = 0.5;

    let z: i32 = x + y as i32;

    let msg: String = String::from("Hello, World!");
    let msg2: String = "Hello, World!".to_string();
    let msg3: &str = "Hello, World!";

    let msg4 = format!("Point: {}, {}", x, y);

    println!("{}", msg4)

}
