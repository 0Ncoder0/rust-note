// use for reference before familiar with rust

fn main() {
    let mut x = 0;
    println!("The value of x is: {}", x);
    x = 1;
    println!("The value of x is: {}", x);
    // -----------------------------
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    // -----------------------------
    let x = String::from("hello");
    let y = x; // value moved here, x no longer valid
    // println!("The value of x is: {}", x); // error: value borrowed here after move
    println!("The value of y is: {}", y); // 123
    // -----------------------------
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // -----------------------------
    let x = (500, 6.4, 1);
    let y: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of x is: {:?}", x);
    println!("The value of y is: {:?}", y);
    let five_hundred = x.0;
    println!("The value of five_hundred is: {}", five_hundred);
    let (x, y, z) = y;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
    // -----------------------------
    let x = [1, 2, 3, 4, 5];
    println!("The value of x is: {:?}", x);
    let x = [3; 5]; // same as let x = [3, 3, 3, 3, 3];
    println!("The value of x is: {:?}", x);
    let x = x[1];
    println!("The value of x is: {}", x); // 3
    // -----------------------------
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number); // 5
    // -----------------------------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 20
        }
    };
    println!("The value of result is: {}", result);
    // -----------------------------
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    // -----------------------------
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value of element is: {}", element);
    }
    for i in 1..4 {
        println!("{}!", i); // 1, 2, 3
    }
    // -----------------------------
    for (index, value) in [1, 2, 3].iter().enumerate() {
        println!("The value of index, value is: {}, {}", index, value);
    }
    // -----------------------------
}
