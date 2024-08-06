fn main() {
    // println!("Hello, world!");
    print!("Hello, ");
    print!("Rust!");

    println!("Hello, {}", "everyone");
    println!("Hello, everyone");

    let a: i32 = 1;
    println!("{}", a);

    let mut b: i32 = 2;
    b = 10;
    println!("{}", b);

    let cc: i32 = 1;

    let d: i32;
    d = 1;
    println!("{}", d);

    let e: &str = "hello";
    println!("{}", e);

    let x: i32 = 1;
    let y: f64 = 2.0;

    let c: u16 = 3;

    let n: f32 = 4.0f32;

    let e: i32 = 1 + 2;
    let e: i32 = 1 - 2;
    let e: i32 = 1 * 2;
    let e: i32 = 1 / 2;


    let f: f64 = 1 as f64 + 2.0;

    let g: bool = false;

    println!("{}", g);

}

// Bはグローバルスコープ、どこからも参照できるようになる
const B: i32 = 2;