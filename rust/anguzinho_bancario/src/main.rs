fn get_money_from_user() -> String {
    let mut total_money: String = String::new();
    println!("Type the quantity: ");
    std::io::stdin()
        .read_line(&mut total_money)
        .expect("Invalid Input");

    let total_money_f: f64 = total_money.trim().parse().expect("Invalid Input");
    let total_money_str: String = format!("{:.2}", total_money_f);
    total_money_str
}

fn separate_in_coins_and_notes(total_money: String) -> (String, String) {
    let mut point_found: bool = false;
    let mut coins_value: String = String::from("");
    let mut notes_value: String = String::from("");
    for character in total_money.chars() {
        if character == '.' {
            point_found = true
        } else if point_found {
            coins_value.push(character)
        } else {
            notes_value.push(character)
        }
    }
    (notes_value, coins_value)
}

fn print_amount_of_notes_on_total_money(total_notes_money: i32, type_of_notes: Vec<i32>) {
    let mut rest_of_notes_money: i32 = total_notes_money;
    println!("NOTAS:");
    for note in type_of_notes {
        let quantity_of_notes: i32 = rest_of_notes_money / note;
        rest_of_notes_money %= note;
        if note == 1 {
            println!("MOEDAS:");
            println!("{} moedas de R${},00", quantity_of_notes, note)
        } else {
            println!("{} notas de R${},00", quantity_of_notes, note)
        }
    }
}

fn print_amount_of_coins_on_total_money(total_coins_money: i32, types_of_coins: Vec<i32>) {
    let mut rest_of_coins_money: i32 = total_coins_money;
    for coin in types_of_coins {
        let quantity_of_coins: i32 = rest_of_coins_money / coin;
        rest_of_coins_money %= coin;
        let fcoin = coin as f64;
        let real_coin_value = fcoin / 100.0;
        println!("{} moedas de R${:.2}", quantity_of_coins, real_coin_value)
    }
}

fn main() {
    let total_money: String = get_money_from_user();
    let (total_notes_money_str, total_coins_money_str): (String, String) =
        separate_in_coins_and_notes(total_money);
    let total_notes_money: i32 = total_notes_money_str.parse().expect("Invalid Input");
    let total_coins_money: i32 = total_coins_money_str.parse().expect("Invalid Input");
    let type_of_notes = vec![100, 50, 20, 10, 5, 2, 1];
    let type_of_coins = vec![50, 25, 10, 5, 1];
    print_amount_of_notes_on_total_money(total_notes_money, type_of_notes);
    print_amount_of_coins_on_total_money(total_coins_money, type_of_coins);
}
