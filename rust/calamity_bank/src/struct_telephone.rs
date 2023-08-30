#[derive(PartialEq, Eq, Debug, Clone,serde::Serialize, serde::Deserialize)]
pub struct Telephone {
    pub ddd: String,
    pub phone_number: String,
}

#[derive(PartialEq, Eq, Debug)]

pub enum TelephoneValidationErrors {
    NoDDDInputDetected,
    InvalidDDDInput,
    InvalidPhoneNumberInput,
    NoMobilePhoneValidationDigitDetected,
    NoPhoneNumberInputDetected,
    NoTelephoneInputDetected,
}

impl Telephone {
    pub fn get_valid_telephone(number: String) -> Result<Telephone, TelephoneValidationErrors> {
        let user_telephone = Telephone {
            ddd: match Telephone::get_valid_ddd(&number) {
                Ok(ddd) => ddd,
                Err(error) => return Err(error),
            },
            phone_number: match Telephone::get_valid_phone_number(&number) {
                Ok(phone_number) => phone_number,
                Err(error) => return Err(error),
            },
        };
        Ok(user_telephone)
    }

    fn get_valid_ddd(number: &String) -> Result<String, TelephoneValidationErrors> {
        if number.is_empty() {
            return Err(TelephoneValidationErrors::NoTelephoneInputDetected);
        }
        if number.len() != 11 {
            return Err(TelephoneValidationErrors::NoDDDInputDetected);
        }
        let mut ddd: String = String::new();
        let mut starting_index: usize = 0;
        while starting_index < 2 {
            let ddd_number: char = match number.chars().nth(starting_index) {
                Some(number) if !number.is_numeric() => {
                    return Err(TelephoneValidationErrors::InvalidDDDInput)
                }
                Some(number) => number,
                None => return Err(TelephoneValidationErrors::NoDDDInputDetected),
            };
            ddd.push(ddd_number);
            starting_index += 1;
        }
        Ok(ddd.to_string())
    }

    fn get_valid_phone_number(number: &String) -> Result<String, TelephoneValidationErrors> {
        if number.is_empty() {
            return Err(TelephoneValidationErrors::NoTelephoneInputDetected);
        }
        if number.len() != 11 {
            return Err(TelephoneValidationErrors::NoDDDInputDetected);
        }
        let mut phone_number: String = String::new();
        let mut starting_index: usize = 2;
        while starting_index < number.len() {
            let phone_number_number: char = match number.chars().nth(starting_index) {
                Some(number) if !number.is_numeric() => {
                    return Err(TelephoneValidationErrors::InvalidPhoneNumberInput)
                }
                Some(number) if starting_index == 2 && number != '9' => {
                    return Err(TelephoneValidationErrors::NoMobilePhoneValidationDigitDetected)
                }
                Some(number) => number,
                None => return Err(TelephoneValidationErrors::NoPhoneNumberInputDetected),
            };
            phone_number.push(phone_number_number);
            starting_index += 1;
        }
        Ok(phone_number.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TELEPHONE DDD VALIDATIONS

    #[test]
    fn check_if_fn_get_ddd_works_if_telephone_is_correct() {
        // Arrange
        let number: String = String::from("34992437871");
        let expected_ddd: Result<String, TelephoneValidationErrors> = Ok(String::from("34"));
        // Act
        let to_validate_ddd: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_ddd(&number);
        // Assert
        assert_eq!(expected_ddd, to_validate_ddd);
    }

    #[test]
    fn check_if_fn_get_ddd_returns_correct_error_if_tel_number_has_no_ddd() {
        // Arrange
        let number: String = String::from("992437871");
        let expected_error: Result<String, TelephoneValidationErrors> =
            Err(TelephoneValidationErrors::NoDDDInputDetected);
        // Act
        let to_validate_ddd_error: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_ddd(&number);
        // Assert
        assert_eq!(expected_error, to_validate_ddd_error);
    }

    #[test]
    fn check_if_fn_get_ddd_returns_correct_error_if_there_is_no_user_input() {
        // Arrange
        let number: String = String::from("");
        let expected_error: Result<String, TelephoneValidationErrors> =
            Err(TelephoneValidationErrors::NoTelephoneInputDetected);
        // Act
        let to_validate_ddd_error: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_ddd(&number);
        // Assert
        assert_eq!(expected_error, to_validate_ddd_error);
    }

    #[test]
    fn check_if_fn_get_ddd_returns_correct_error_if_ddd_number_has_a_invalid_character() {
        // Arrange
        let number: String = String::from("3a992437871");
        let expected_error: Result<String, TelephoneValidationErrors> =
            Err(TelephoneValidationErrors::InvalidDDDInput);
        // Act
        let to_validate_ddd_error: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_ddd(&number);
        // Assert
        assert_eq!(expected_error, to_validate_ddd_error);
    }

    // TELEPHONE NUMBER VALIDATIONS

    #[test]
    fn check_if_fn_get_phone_number_works_if_telephone_is_correct() {
        // Arrange
        let number: String = String::from("34992437871");
        let expected_phone_number: Result<String, TelephoneValidationErrors> =
            Ok(String::from("992437871"));
        // Act
        let to_validate_phone_number: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_phone_number(&number);
        // Assert
        assert_eq!(expected_phone_number, to_validate_phone_number);
    }

    #[test]
    fn check_if_fn_get_phone_number_returns_correct_error_if_phone_number_has_a_invalid_character()
    {
        // Arrange
        let number: String = String::from("3499243787i");
        let expected_error: Result<String, TelephoneValidationErrors> =
            Err(TelephoneValidationErrors::InvalidPhoneNumberInput);
        // Act
        let to_validate_phone_number_error: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_phone_number(&number);
        // Assert
        assert_eq!(expected_error, to_validate_phone_number_error);
    }

    #[test]
    fn check_if_fn_get_phone_number_returns_correct_error_if_phone_number_doesnt_have_mobile_number_validation_digit(
    ) {
        // Arrange
        let number: String = String::from("34892437871");
        let expected_error: Result<String, TelephoneValidationErrors> =
            Err(TelephoneValidationErrors::NoMobilePhoneValidationDigitDetected);
        // Act
        let to_validate_phone_number_error: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_phone_number(&number);
        // Assert
        assert_eq!(expected_error, to_validate_phone_number_error);
    }

    #[test]
    fn check_if_fn_get_phone_number_returns_correct_error_if_user_input_is_empty() {
        // Arrange
        let number: String = String::from("");
        let expected_error: Result<String, TelephoneValidationErrors> =
            Err(TelephoneValidationErrors::NoTelephoneInputDetected);
        // Act
        let to_validate_phone_number_error: Result<String, TelephoneValidationErrors> =
            Telephone::get_valid_phone_number(&number);
        // Assert
        assert_eq!(expected_error, to_validate_phone_number_error);
    }

    // TELEPHONE STRUCT CREATION VALIDATIONS

    #[test]
    fn check_if_fn_get_valid_telephone_works_if_telephone_is_correct() {
        // Arrange
        let number: String = String::from("34992437871");
        let expected_phone_number = Ok(Telephone {
            ddd: String::from("34"),
            phone_number: String::from("992437871"),
        });
        // Act
        let to_validate_phone_number: Result<Telephone, TelephoneValidationErrors> =
            Telephone::get_valid_telephone(number);
        // Assert
        assert_eq!(expected_phone_number, to_validate_phone_number);
    }
}
