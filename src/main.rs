fn main() {
    let result = fibonacci_to_n(25);
    println!("{result}");
}

fn fibonacci_to_n(n: i32) -> i32 {
    // (n - 1) + (n - 2)がn番目のフィボナッチ数列の数値
    // 0 1 1 2 3 5 8 13 21 34 55...
    if n <= 0 {
        println!("1以上の数値で入力してください");
        return 0;
    };
    if n == 1 { return 0; };
    if n == 2 { return 1; };

    let mut f1: i32 = 0;
    let mut f2: i32 = 1;
    let mut fibonacci = 0;

    let mut count: i32 = 2;

    while n > count {
        fibonacci = f1 + f2;
        f1 = f2;
        f2 = fibonacci;
        count += 1;
    }
    
    fibonacci
}