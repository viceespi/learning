use crate::struct_account::{Account, AccountCreationErrors, AccountType};
use crate::struct_adress::{Adress, AdressValidationErrors};
use crate::struct_cpf::{Cpf, CpfValidationErrors};
use crate::struct_date_of_birth_v2::{DOBValidationError, Dob};
use crate::struct_email::{Email, EmailValidationError};
use crate::struct_name_v2::{Name, NameValidationError};
use crate::struct_pf::PF;
use crate::struct_telephone::{Telephone, TelephoneValidationErrors};
use core::panic;

pub trait DisplayableOnTerminal {
    fn print_on_terminal(&self);
}

impl DisplayableOnTerminal for Name {
    fn print_on_terminal(&self) {
        println!("User name:\n{} {}", self.first_name, self.last_name)
    }
}

impl DisplayableOnTerminal for Cpf {
    fn print_on_terminal(&self) {
        let mut string_first_nine_numbers: String = String::new();
        let mut string_last_two_numbers: String = String::new();
        for number in &self.first_nine_digits {
            let character: char = match char::from_u32(*number) {
                Some(number) => number,
                None => panic!(""),
            };
            string_first_nine_numbers.push(character);
        }
        for number in &self.last_two_digits {
            let character: char = match char::from_u32(*number) {
                Some(number) => number,
                None => panic!(""),
            };
            string_last_two_numbers.push(character);
        }
        println!(
            "User CPF:\n{}-{}",
            string_first_nine_numbers, string_last_two_numbers
        );
    }
}

impl DisplayableOnTerminal for Dob {
    fn print_on_terminal(&self) {
        println!(
            "User date of birth:\n{}/{}/{}",
            self.day, self.month, self.year
        );
    }
}

impl DisplayableOnTerminal for Email {
    fn print_on_terminal(&self) {
        println!(
            "User email:\n{}@{}{}",
            self.local_part, self.email_domain, self.top_domain
        );
    }
}

impl DisplayableOnTerminal for Telephone {
    fn print_on_terminal(&self) {
        println!("User phone number:\n({}) {}", self.ddd, self.phone_number)
    }
}

impl DisplayableOnTerminal for Adress {
    fn print_on_terminal(&self) {
        println!(
            "User Adress:\nUser postal code: {}\nUser adress number: {}",
            self.postal_code, self.adress_number
        )
    }
}

// FUNCTIONS RELATED TO USER NAME INPUT

fn get_name_from_terminal() -> Name {
    loop {
        println!("\nType the User first name:");
        let mut user_first_name: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_first_name) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
        };
        println!("\nType the User last name:");
        let mut user_last_name: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_last_name) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
        };

        let error_message = match Name::get_valid_user_name(user_first_name, user_last_name) {
            Ok(valid_name) => return valid_name,
            Err(validation_error) => match validation_error {
                NameValidationError::FirstNameStartsWhitLowerCase => "\nFirst name is invalid. First name must start whit uppercase letter.",
                NameValidationError::FirstNameHasInvalidLenght => "\nFirst name is invalid. First name must have between 2 and 15 characters.",
                NameValidationError::FirstNameHasWhiteSpace => "\nFirst name is invalid. First name must not contain any white spaces.",
                NameValidationError::FirstNameHasInvalidCharacter => "\nFirst name is invalid. First name must contain only letters from the alphabet and special characters like '-'.",
                NameValidationError::FirstNameNotDetected => "\nFirst name is invalid. First name wasn't typed.",
                NameValidationError::LastNameStartsWhitLowerCase => "\nLast name is invalid. Last name must start whit uppercase letter.",
                NameValidationError::LastNameHasInvalidLenght => "\nLast name is invalid. Last name must have between 2 and 15 characters.",
                NameValidationError::LastNameHasWhiteSpace => "\nLast name is invalid. Last name must not contain any white spaces.",
                NameValidationError::LastNameHasInvalidCharacter => "\nLast name is invalid. Last name must contain only letters from the alphabet and special characters like '-'.",
                NameValidationError::LastNameNotDetected => "\nLast name is invalid. Last name wasn't typed.",
            },
        };
        println!("{}", error_message);
    }
}

// FUNCTIONS RELATED TO USER CPF
fn get_cpf_string_from_terminal() -> Result<Vec<u32>, String> {
    println!("\nType the User CPF: ");
    let mut user_cpf_string: String = String::new();
    _ = match std::io::stdin().read_line(&mut user_cpf_string) {
        Ok(_) => (),
        Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
    };
    let mut user_cpf: Vec<u32> = vec![];
    for characterer in (user_cpf_string.trim()).chars() {
        let cpf_number: u32 = match characterer.to_digit(10) {
            Some(number) => number,
            None => return Err(String::from("Invalid CPF character.")),
        };
        user_cpf.push(cpf_number)
    }
    Ok(user_cpf)
}

fn get_cpf_as_a_vector() -> Vec<u32> {
    loop {
        let cpf_transformation_error = match get_cpf_string_from_terminal() {
            Ok(cpf) => return cpf,
            Err(_) => "\nInvalid CPF. One of the characters in the CPF isn't a number.",
        };
        println!("{}", cpf_transformation_error)
    }
}
fn get_cpf_from_terminal() -> Cpf {
    loop {
        let user_cpf: Vec<u32> = get_cpf_as_a_vector();
        let cpf_error_message = match Cpf::get_valid_cpf_struct(user_cpf) {
            Ok(cpf) => return cpf,
            Err(cpf_validation_error) => match cpf_validation_error {
                CpfValidationErrors::NoUserCPFInputDetected => "\nCPF is invalid. CPF wasn't typed.",
                CpfValidationErrors::InvalidCPFLenght => "\nCPF is invalid. CPF must have 11 numbers.",
                CpfValidationErrors::InvalidPenultimateDigitOfCPF => {
                    "\nCPF is invalid. Penultimate digit from CPF do not pass the validation process."
                }
                CpfValidationErrors::InvalidLastDigitOfCPF => {
                    "\nCPF is invalid. Last digit from CPF do not pass the validation process."
                }
            },
        };
        println!("{}", cpf_error_message);
    }
}

// FUNCTIONS RELATED TO USER DATE OF BIRTH

fn get_user_year_of_birth_from_terminal() -> i32 {
    loop {
        println!("\nDate of birth informations");
        println!("\nType the year of birth: ");
        let mut user_year_of_birth_string: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_year_of_birth_string) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
        };
        let parse_error = match (user_year_of_birth_string.trim()).parse() {
            Ok(year) => return year,
            Err(parse_error) => "\nInvalid date of birth. Year of birth must contain only numbers.",
        };
        println!("{}", parse_error);
    }
}

fn get_user_month_of_birth_from_terminal() -> i32 {
    loop {
        println!("\nType the month of birth: ");
        let mut user_month_of_birth_string: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_month_of_birth_string) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
        };
        let parse_error = match (user_month_of_birth_string.trim()).parse() {
            Ok(year) => return year,
            Err(parse_error) => {
                "\nInvalid date of birth. Month of birth must contain only numbers."
            }
        };
        println!("{}", parse_error);
    }
}

fn get_user_day_of_birth_from_terminal() -> i32 {
    loop {
        println!("\nType the day of birth: ");
        let mut user_day_of_birth_string: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_day_of_birth_string) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
        };
        let parse_error = match (user_day_of_birth_string.trim()).parse() {
            Ok(year) => return year,
            Err(parse_error) => "\nInvalid date of birth. Day of birth must contain only numbers.",
        };
        println!("{}", parse_error);
    }
}

fn get_dob_from_terminal() -> Dob {
    loop {
        let user_year_of_birth: i32 = get_user_year_of_birth_from_terminal();
        let user_month_of_birth: i32 = get_user_month_of_birth_from_terminal();
        let user_day_of_birth: i32 = get_user_day_of_birth_from_terminal();
        let dob_error = match Dob::get_valid_date_of_birth(
            user_year_of_birth,
            user_month_of_birth,
            user_day_of_birth,
        ) {
            Ok(dob) => return dob,
            Err(dob_validation_error) => match dob_validation_error {
                DOBValidationError::InvalidAge => {
                    "\nInvalid date of birth. User must be at least 18 years old and no more than 115 years old."
                }
                DOBValidationError::InvalidMonth => {
                    "\nInvalid date of birth. Month must be between january (1) and december (12)."
                }
                DOBValidationError::InvalidDay => "\nInvalid date of birth. The chosen day is invalid.",
                DOBValidationError::NoUserInputDetected => {
                    "\nInvalid date of birth. Date of birth wasn't typed."
                }
            },
        };
        println!("{}", dob_error);
    }
}
// FUNCTIONS RELATED TO USER EMAIL
fn get_email_from_terminal() -> Email {
    loop {
        println!("\nType the email for registration.");
        let mut user_email: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_email) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input."),
        };
        let email_error = match Email::get_valid_email((user_email.trim()).to_string()) {
            Ok(email) => return email,
            Err(email_validation_error) => match email_validation_error {
                EmailValidationError::NoAtDetected => "\nInvalid email. @ wasn't typed.",
                EmailValidationError::NoLocalPartDetected => "\nInvalid email. Local part wasn't typed.",
                EmailValidationError::InvalidLocalPartCharacter => "\nInvalid email. Local part must contain only alphanumerics and certain special characters.",
                EmailValidationError::SpecialCharInARowInLocalPart => "\nInvalid email. Local part can't have special characters in a row.",
                EmailValidationError::InvalidStartEndCharInLocalPart => "\nInvalid email. Local part can't start or end whit special characters.",
                EmailValidationError::NoEmailDomainDetected => "\nInvalid email.  Domain part wasn't typed.",
                EmailValidationError::InvalidEmailDomainCharacter => "\nInvalid email. Domain part must contain only alphanumerics and certain special characters.",
                EmailValidationError::InvalidEmailDomainStartEndCharacter => "\nInvalid email. Domain part can't start or end whit special characters.",
                EmailValidationError::SpecialCharInARowInEmailDomain => "\nInvalid email. Domain part can't have special characters in a row.",
                EmailValidationError::NoTopDomainDetected => "\nInvalid email. Domain part wasn't typed.",
                EmailValidationError::TopDomainLengthInvalid => "\nInvalid email. Domain part has a invalid lenght.",
                EmailValidationError::InvalidTopDomainCharacter => "\nInvalid email. Domain part must contain only alphanumerics and certain special characters.",
            }
        };
        println!("{}", email_error);
    }
}

// FUNCTIONS RELATED TO USER TELEPHONE

fn get_telephone_from_terminal() -> Telephone {
    loop {
        println!("\nType the user telephone: ");
        let mut user_telephone: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_telephone) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading the user input"),
        };
        let telephone_error = match Telephone::get_valid_telephone((user_telephone.trim()).to_string()) {
            Ok(valid_telephone) => return valid_telephone,
            Err(telephone_validation_error) => match telephone_validation_error {
                TelephoneValidationErrors::NoDDDInputDetected => "\nInvalid telephone. DDD wasn't typed.",
                TelephoneValidationErrors::InvalidDDDInput => "\nInvalid telephone. Telephone must contain only numbers.",
                TelephoneValidationErrors::InvalidPhoneNumberInput => "\nInvalid telephone. Telephone must contain only numbers.",
                TelephoneValidationErrors::NoMobilePhoneValidationDigitDetected => "\nInvalid telephone. Telephone number must be preceeded by mobile phone validation digit (9)",
                TelephoneValidationErrors::NoPhoneNumberInputDetected => "\nInvalid telephone. Phone number wasn't typed.",
                TelephoneValidationErrors::NoTelephoneInputDetected => "\nInvalid telephone. Full telephone number wasn't typed.",
            }
        };
        println!("{}", telephone_error);
    }
}

// FUNCTIONS RELATED TO USER ADRESS

fn get_adress_from_terminal() -> Adress {
    loop {
        println!("\nPlease type the adress postal code: ");
        let mut user_adress_postal_code: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_adress_postal_code) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading user input"),
        };
        println!("\nPlease type the adress number");
        let mut user_adress_number: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_adress_number) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading user input"),
        };
        let adress_error = match Adress::create_user_adress(
            (user_adress_number.trim()).to_string(),
            (user_adress_postal_code.trim()).to_string(),
        ) {
            Ok(adress) => return adress,
            Err(adress_validation_error) => match adress_validation_error {
                AdressValidationErrors::NoPostalCodeInputDetected => {
                    "\nInvalid adress. Postal code wasn't typed."
                }
                AdressValidationErrors::PostalCodeInvalidCharacter => {
                    "\nInvalid adress. Postal code must contain only numbers."
                }
                AdressValidationErrors::PostalCodeInvalidLenght => {
                    "\nInvalid adress. Postal code must contain exactly 7 numbers."
                }
            },
        };
        println!("{}", adress_error);
    }
}

// FUNCTIONS RELATED TO GETTING THE USER DESIRED TYPE OF ACCOUNT

fn get_user_account_type_from_terminal() -> AccountType {
    loop {
        println!(
            "\nWhat type of account you're interested in?\n1- Corrente\n2- Poupança\n3- Salário"
        );
        let mut user_account_choice: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_account_choice) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading user input."),
        };
        let type_of_account_error =
            match Account::get_user_desired_account((user_account_choice.trim()).to_string()) {
                Ok(account_type) => return account_type,
                Err(account_type_error) => match account_type_error {
                    crate::struct_account::AccountTypeError::NoUserInputDetected => {
                        "\nUser must select a type of account."
                    }
                },
            };
        println!("{}", type_of_account_error);
    }
}

// FUNCTIONS RELATED TO USER ACCOUNT PASSWORD

fn get_user_password_from_terminal() -> String {
    loop {
        println!("\nType the account password: ");
        let mut user_password: String = String::new();
        _ = match std::io::stdin().read_line(&mut user_password) {
            Ok(_) => (),
            Err(_) => panic!("\nThere was an unexpected error when reading user input."),
        };
        let password_validation_error = match Account::validate_user_password((user_password.trim()).to_string()) {
            Ok(password) => return password,
            Err(password_validation_errors) => match password_validation_errors {
                crate::struct_account::AccountPasswordErrors::PasswordHasNoSpecialCharacter => "\nPassword is invalid. The password needs at least 1 special character (!,@,#,$,%,&,*,-,+,_)",
                crate::struct_account::AccountPasswordErrors::PasswordHasNoNumbers => "\nPassword is invalid. The password needs at least 1 number.",
                crate::struct_account::AccountPasswordErrors::PasswordHasNoUpperCase => "\nPassword is invalid. The password needs at least 1 letter in uppercase.",
                crate::struct_account::AccountPasswordErrors::PasswordHasNoLowerCase => "\nPassword is invalid. The password needs at least 1 letter in lowercase.",
                crate::struct_account::AccountPasswordErrors::PasswordInvalidLenght => "\nPassword is invalid. The password must be between 8 and 20 characters long.",
            }
        };
        println!("{}", password_validation_error)
    }
}

// FUNCTIONS RELATED TO THE CREATION OF A PF

fn create_user_pf() -> PF {
    let user_name: Name = get_name_from_terminal();
    let user_cpf: Cpf = get_cpf_from_terminal();
    let user_date_of_birth: Dob = get_dob_from_terminal();
    let user_email: Email = get_email_from_terminal();
    let user_telephone: Telephone = get_telephone_from_terminal();
    let user_adress: Adress = get_adress_from_terminal();
    let user_pf: PF = crate::struct_pf::PF::create_new_pf(
        user_name,
        user_cpf,
        user_date_of_birth,
        user_email,
        user_telephone,
        user_adress,
    );
    user_pf
}
// FUNCTIONS RELATED TO THE CREATION OF AN ACCOUNT
pub fn create_user_account_from_terminal_input() -> Account {
    let user_pf: PF = create_user_pf();
    let user_account_type: AccountType = get_user_account_type_from_terminal();
    let user_password: String = get_user_password_from_terminal();
    let user_account: Account =
        Account::create_user_account(user_pf, user_account_type, user_password);
    user_account
}
