fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    another_function();
}

fn another_function() {
    println!("Another function.");  // 別の関数
}
