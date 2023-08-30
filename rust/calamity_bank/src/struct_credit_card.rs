use crate::struct_account::Account;
use rand::Rng;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreditCard {
    account_name: String,
    card_number: u64,
    expiration_date: String,
    security_code: i16,
}

impl CreditCard {
    fn generate_random_card_number() -> u64 {
        let mut rng = rand::thread_rng();
        let credit_card_number: u64 = rng.gen_range(1000000000000000..=9999999999999999);
        credit_card_number
    }

    fn generate_random_security_code() -> i16 {
        let mut rng = rand::thread_rng();
        let security_code: i16 = rng.gen_range(100..=999);
        security_code
    }

    fn get_expiration_date(current_month: i16, current_year: i32) -> String {
        let expiration_month: String = current_month.to_string();
        let expiration_year: String = (current_year + 8).to_string();
        let expiration_date: String = expiration_month + &String::from("/") + &expiration_year;
        expiration_date
    }

    fn create_credit_card(
        current_month: i16,
        current_year: i32,
        user_account: Account,
    ) -> CreditCard {
        let user_credit_card: CreditCard = CreditCard {
            account_name: user_account.user_type.name.first_name
                + &String::from(" ")
                + &user_account.user_type.name.last_name,
            card_number: CreditCard::generate_random_card_number(),
            expiration_date: CreditCard::get_expiration_date(current_month, current_year),
            security_code: CreditCard::generate_random_security_code(),
        };
        user_credit_card
    }
}
