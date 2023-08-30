mod struct_account;
use struct_account::Account;
mod accounts_data_manipulation;
mod struct_adress;
mod struct_cpf;
mod struct_credit_card;
mod struct_date_of_birth_v2;
mod struct_email;
mod struct_name_v2;
mod struct_pf;
mod struct_telephone;
mod terminal_user_input;
use csv;
use std::fs;
use terminal_user_input::DisplayableOnTerminal;

fn main() {
    let current_day: i16 = 23;
    let current_month: i16 = 8;
    let current_year: i32 = 2023;
    let mut accounts: Vec<Account> = vec![];
    let mut intro_prompt: String =
        String::from("\nWelcome to Calamity Bank!\nWhat would you like to do?");
    loop {
        println!(
            "{}\nPress 1 to login.\nPress 2 to create an account.\nPress 3 to exit the app.",
            intro_prompt
        );
        let mut user_option: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_option) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input"),
        };
        let trimmed_user_option: &str = user_option.trim();
        match trimmed_user_option {
            "1" => {
                println!("\nFunction not implemented yet, closing program.");
                break;
            }
            "2" => {
                let user_account: Account =
                    crate::terminal_user_input::create_user_account_from_terminal_input();
                println!(
                    "\n{} {}, your account was successfully created!",
                    user_account.user_type.name.first_name, user_account.user_type.name.last_name
                );
                intro_prompt = String::from(
                    "\nThank you for using Calamity Bank!\nWhat would you like to do now?",
                );
            }
            "3" => {
                println!("\nThank you for using Calamity Bank!");
                break;
            }
            &_ => {
                println!("\nPlease, choose a valid option!");
            }
        }
    }
}
