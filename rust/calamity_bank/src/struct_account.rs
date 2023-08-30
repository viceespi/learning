use crate::struct_cpf::Cpf;
use crate::struct_credit_card::CreditCard;
use crate::struct_pf::PF;
use rand::Rng;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub user_type: PF,
    account_type: AccountType,
    account_login: Cpf,
    account_password: String,
    account_number: u32,
    agency: u32,
    balance: f64,
    credit_card: Option<CreditCard>,
}

pub enum AccountTypeError {
    NoUserInputDetected,
}

pub enum AccountPasswordErrors {
    PasswordInvalidLenght,
    PasswordHasNoSpecialCharacter,
    PasswordHasNoNumbers,
    PasswordHasNoUpperCase,
    PasswordHasNoLowerCase,
}
pub enum AccountCreationErrors {
    AccountTypeError(AccountTypeError),
    PasswordValidationError(AccountPasswordErrors),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AccountType {
    Corrente,
    Salario,
    Poupanca,
}

impl Account {
    pub fn get_user_desired_account(
        user_account_choice: String,
    ) -> Result<AccountType, AccountTypeError> {
        if user_account_choice == "1" {
            return Ok(AccountType::Corrente);
        }
        if user_account_choice == "2" {
            return Ok(AccountType::Poupanca);
        }
        if user_account_choice == "3" {
            Ok(AccountType::Salario)
        } else {
            Err(AccountTypeError::NoUserInputDetected)
        }
    }

    pub fn validate_user_password(user_password: String) -> Result<String, AccountPasswordErrors> {
        if !user_password.contains('!')
            && !user_password.contains('@')
            && !user_password.contains('#')
            && !user_password.contains('$')
            && !user_password.contains('%')
            && !user_password.contains('&')
            && !user_password.contains('*')
            && !user_password.contains('-')
            && !user_password.contains('+')
            && !user_password.contains('_')
        {
            return Err(AccountPasswordErrors::PasswordHasNoSpecialCharacter);
        }
        if user_password.len() < 8 || user_password.len() > 20 {
            return Err(AccountPasswordErrors::PasswordInvalidLenght);
        }
        let mut _has_number: bool = false;
        let mut _has_upper_case: bool = false;
        let mut _has_lower_case: bool = false;
        for character in user_password.chars() {
            if character.is_numeric() {
                _has_number = true;
            }
            if character.is_uppercase() {
                _has_upper_case = true
            }
            if character.is_lowercase() {
                _has_lower_case = true;
            }
        }
        match (_has_number, _has_upper_case, _has_lower_case) {
            (true, true, true) => Ok(user_password),
            (false, _, _) => Err(AccountPasswordErrors::PasswordHasNoNumbers),
            (_, false, _) => Err(AccountPasswordErrors::PasswordHasNoUpperCase),
            (_, _, false) => Err(AccountPasswordErrors::PasswordHasNoLowerCase),
        }
    }

    fn generate_account_number() -> u32 {
        let mut rng = rand::thread_rng();
        let account_number: u32 = rng.gen_range(100000..=999999);
        account_number
    }

    pub fn create_user_account(
        user_pf: PF,
        user_account_type: AccountType,
        user_password: String,
    ) -> Account {
        let user_account: Account = Account {
            user_type: user_pf.clone(),
            account_type: user_account_type,
            account_login: user_pf.cpf,
            account_password: user_password,
            account_number: Account::generate_account_number(),
            agency: 0,
            balance: 0.00,
            credit_card: None,
        };
        user_account
    }
}
