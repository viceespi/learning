fn main() {
    let num1: i32 = usernum();
    let num2: i32 = usernum();
    let sum: i32 = sum(num1, num2);
    println!("A soma é {}", sum);
}

fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn usernum() -> i32 {
    let mut input: String = String::new();
    println!("Digite o número: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error, invalid input");
    let number: i32 = input.trim().parse().expect("Invalid input, type a number");
    number
}
