#[derive(PartialEq, Eq, Debug,Clone,serde::Serialize, serde::Deserialize)]
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}

#[derive(PartialEq, Eq, Debug)]

pub enum NameValidationError {
    FirstNameStartsWhitLowerCase,
    FirstNameHasInvalidLenght,
    FirstNameHasWhiteSpace,
    FirstNameHasInvalidCharacter,
    FirstNameNotDetected,
    LastNameStartsWhitLowerCase,
    LastNameHasInvalidLenght,
    LastNameHasWhiteSpace,
    LastNameHasInvalidCharacter,
    LastNameNotDetected,
}

impl Name {
    fn get_valid_first_name(name: String) -> Result<String, NameValidationError> {
        let check_name: String = name.trim().to_string();
        if check_name.is_empty() {
            return Err(NameValidationError::FirstNameNotDetected);
        };
        if check_name.len() <= 1 || check_name.len() > 15 {
            return Err(NameValidationError::FirstNameHasInvalidLenght);
        };
        for character in check_name.chars() {
            if character == ' ' || character == '\n' || character == '\t' {
                return Err(NameValidationError::FirstNameHasWhiteSpace);
            }
            if !character.is_alphabetic() && character != '-' {
                return Err(NameValidationError::FirstNameHasInvalidCharacter);
            }
        }
        let check_name_first_letter: char = match check_name.chars().nth(0) {
            Some(character) => character,
            None => return Err(NameValidationError::FirstNameNotDetected),
        };
        if check_name_first_letter.is_lowercase() {
            return Err(NameValidationError::FirstNameStartsWhitLowerCase);
        }
        Ok(check_name)
    }

    fn get_valid_last_name(name: String) -> Result<String, NameValidationError> {
        let check_name: String = name.trim().to_string();
        if check_name.is_empty() {
            return Err(NameValidationError::LastNameNotDetected);
        };
        if check_name.len() <= 1 || check_name.len() > 15 {
            return Err(NameValidationError::LastNameHasInvalidLenght);
        };
        for character in check_name.chars() {
            if character == ' ' || character == '\n' || character == '\t' {
                return Err(NameValidationError::LastNameHasWhiteSpace);
            }
            if !character.is_alphabetic() {
                return Err(NameValidationError::LastNameHasInvalidCharacter);
            }
        }
        let check_name_first_letter: char = match check_name.chars().nth(0) {
            Some(character) => character,
            None => return Err(NameValidationError::LastNameNotDetected),
        };
        if check_name_first_letter.is_lowercase() {
            return Err(NameValidationError::LastNameStartsWhitLowerCase);
        }
        Ok(check_name)
    }

    pub fn get_valid_user_name(
        user_first_name: String,
        user_last_name: String,
    ) -> Result<Name, NameValidationError> {
        let user_name = Name {
            first_name: match Name::get_valid_first_name(user_first_name) {
                Ok(name) => name,
                Err(error) => return Err(error),
            },
            last_name: match Name::get_valid_last_name(user_last_name) {
                Ok(name) => name,
                Err(error) => return Err(error),
            },
        };
        Ok(user_name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fn_get_valid_name_if_the_input_is_valid() {
        // Arrange

        let name: String = String::from("Thedeuzinho");
        let expected_name: Result<String, NameValidationError> = Ok(String::from("Thedeuzinho"));

        // Act

        let to_validate_name: Result<String, NameValidationError> =
            Name::get_valid_first_name(name);

        // Assert

        assert_eq!(expected_name, to_validate_name);
    }

    #[test]
    fn test_fn_get_valid_name_if_error_is_name_starts_whit_lower_case() {
        // Arrange

        let name: String = String::from("nenega");
        let expected_name_validation_error: Result<String, NameValidationError> =
            Err(NameValidationError::LastNameStartsWhitLowerCase);

        // Act

        let to_validate_name_error: Result<String, NameValidationError> =
            Name::get_valid_first_name(name);

        // Assert

        assert_eq!(expected_name_validation_error, to_validate_name_error);
    }

    #[test]
    fn test_fn_get_valid_name_if_error_is_name_has_white_space() {
        // Arrange

        let name: String = String::from("Po po cão");
        let expected_name_validation_error: Result<String, NameValidationError> =
            Err(NameValidationError::LastNameHasWhiteSpace);

        // Act

        let to_validate_name_error: Result<String, NameValidationError> =
            Name::get_valid_first_name(name);

        // Assert

        assert_eq!(expected_name_validation_error, to_validate_name_error);
    }

    #[test]
    fn test_fn_get_valid_name_if_error_is_name_has_invalid_character() {
        // Arrange

        let name: String = String::from("Anguz1nho");
        let expected_name_validation_error: Result<String, NameValidationError> =
            Err(NameValidationError::LastNameHasInvalidCharacter);

        // Act

        let to_validate_name_error: Result<String, NameValidationError> =
            Name::get_valid_first_name(name);

        // Assert

        assert_eq!(expected_name_validation_error, to_validate_name_error);
    }

    #[test]
    fn test_fn_get_valid_name_if_error_is_name_has_invalid_lenght() {
        // Arrange

        let name: String = String::from("Y");
        let expected_name_validation_error: Result<String, NameValidationError> =
            Err(NameValidationError::LastNameHasInvalidLenght);

        // Act

        let to_validate_name_error: Result<String, NameValidationError> =
            Name::get_valid_first_name(name);

        // Assert

        assert_eq!(expected_name_validation_error, to_validate_name_error);
    }

    #[test]
    fn test_fn_get_valid_name_if_error_is_name_not_detected() {
        // Arrange

        let name: String = String::from("");
        let expected_name_validation_error: Result<String, NameValidationError> =
            Err(NameValidationError::LastNameNotDetected);

        // Act

        let to_validate_name_error: Result<String, NameValidationError> =
            Name::get_valid_first_name(name);

        // Assert

        assert_eq!(expected_name_validation_error, to_validate_name_error);
    }

    #[test]
    fn test_if_fn_get_valid_user_name_is_working_if_user_input_is_valid() {
        // Arrange

        let user_first_name: String = String::from("Ana-Maria");
        let user_last_name: String = String::from("Lima");
        let expected_user_name: Result<Name, NameValidationError> = Ok(Name {
            first_name: String::from("Ana-Maria"),
            last_name: String::from("Lima"),
        });

        // Act

        let to_validate_user_name: Result<Name, NameValidationError> =
            Name::get_valid_user_name(user_first_name, user_last_name);

        //Assert

        assert_eq!(expected_user_name, to_validate_user_name);
    }
}
