#[derive(PartialEq, Eq, Debug,Clone,serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub local_part: String,
    pub email_domain: String,
    pub top_domain: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum EmailValidationError {
    NoAtDetected,
    NoLocalPartDetected,
    InvalidLocalPartCharacter,
    SpecialCharInARowInLocalPart,
    InvalidStartEndCharInLocalPart,
    NoEmailDomainDetected,
    InvalidEmailDomainCharacter,
    InvalidEmailDomainStartEndCharacter,
    SpecialCharInARowInEmailDomain,
    NoTopDomainDetected,
    TopDomainLengthInvalid,
    InvalidTopDomainCharacter,
}

impl EmailValidationError {}

impl Email {
    pub fn get_valid_email(email: String) -> Result<Email, EmailValidationError> {
        let user_email: Email = Email {
            local_part: match Email::get_local_part(&email) {
                Ok(string) => string,
                Err(error) => return Err(error),
            },
            email_domain: match Email::get_email_domain(&email) {
                Ok(string) => string,
                Err(error) => return Err(error),
            },
            top_domain: match Email::get_top_domain(&email) {
                Ok(string) => string,
                Err(error) => return Err(error),
            },
        };
        Ok(user_email)
    }

    fn get_local_part(email: &String) -> Result<String, EmailValidationError> {
        let at_index: usize = match email.find('@') {
            Some(index) => index,
            None => return Err(EmailValidationError::NoAtDetected),
        };
        if at_index == 0 {
            return Err(EmailValidationError::NoLocalPartDetected);
        }
        let local_part: String = match email.get(0..at_index) {
            Some(part) => part.to_string(),
            None => return Err(EmailValidationError::NoLocalPartDetected),
        };
        if local_part.starts_with('.') || local_part.ends_with('.') {
            return Err(EmailValidationError::InvalidStartEndCharInLocalPart);
        }

        let mut special_char_switch: bool = false;
        for element in local_part.chars() {
            if !element.is_alphanumeric() && element != '.' && element != '-' && element != '_' {
                return Err(EmailValidationError::InvalidLocalPartCharacter);
            } else if element == '.' || element == '-' || element == '_' {
                if special_char_switch {
                    return Err(EmailValidationError::SpecialCharInARowInLocalPart);
                }
                special_char_switch = true;
            } else {
                special_char_switch = false;
            }
        }

        Ok(local_part)
    }

    fn get_email_domain(email: &String) -> Result<String, EmailValidationError> {
        let at_index: usize = match email.find('@') {
            Some(index) => index,
            None => return Err(EmailValidationError::NoAtDetected),
        };
        if at_index == (email.len() - 1) {
            return Err(EmailValidationError::NoEmailDomainDetected);
        }
        let last_period_index: usize = match email.rfind('.') {
            Some(period_index) if period_index < at_index => email.len(),
            Some(period_index) => period_index,
            None => email.len(),
        };
        let email_domain: String = match email.get((at_index + 1)..last_period_index) {
            Some(part) if part.is_empty() => {
                return Err(EmailValidationError::NoEmailDomainDetected)
            }
            Some(part) => part.to_string(),
            None => return Err(EmailValidationError::NoEmailDomainDetected),
        };
        if email_domain.starts_with('-') || email_domain.ends_with('-') {
            return Err(EmailValidationError::InvalidEmailDomainStartEndCharacter);
        }
        let mut special_char_switch: bool = false;
        for element in email_domain.chars() {
            if !element.is_alphanumeric() && element != '-' {
                return Err(EmailValidationError::InvalidEmailDomainCharacter);
            } else if element == '-' {
                if special_char_switch {
                    return Err(EmailValidationError::SpecialCharInARowInEmailDomain);
                }
                special_char_switch = true;
            } else {
                special_char_switch = false;
            }
        }

        Ok(email_domain)
    }

    fn get_top_domain(email: &String) -> Result<String, EmailValidationError> {
        let at_index: usize = match email.find('@') {
            Some(index) => index,
            None => return Err(EmailValidationError::NoAtDetected),
        };
        if at_index == (email.len() - 1) {
            return Err(EmailValidationError::NoTopDomainDetected);
        }
        let last_period_index: usize = match email.rfind('.') {
            Some(period_index) if period_index < at_index => {
                return Err(EmailValidationError::NoTopDomainDetected)
            }
            Some(period_index) if period_index == (email.len() - 1) => {
                return Err(EmailValidationError::NoTopDomainDetected)
            }
            Some(period_index) => period_index,
            None => return Err(EmailValidationError::NoTopDomainDetected),
        };
        let top_domain: String = match email.get(last_period_index..email.len()) {
            Some(part) if part.is_empty() => return Err(EmailValidationError::NoTopDomainDetected),
            Some(part) => part.to_string(),
            None => return Err(EmailValidationError::NoTopDomainDetected),
        };
        if 3 > top_domain.len() || top_domain.len() > 8 {
            return Err(EmailValidationError::TopDomainLengthInvalid);
        }
        for character in top_domain.chars() {
            if !character.is_alphabetic() && character != '.' {
                return Err(EmailValidationError::InvalidTopDomainCharacter);
            }
        }
        Ok(top_domain)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // LOCAL PART ERRORS TESTS

    #[test]
    fn test_fn_get_local_part_if_it_is_right() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@gmail.com");
        let expected_local_part: Result<String, EmailValidationError> =
            Ok(String::from("Vikenzoespindolaa"));

        // Act
        let to_validate_local_part: Result<String, EmailValidationError> =
            Email::get_local_part(&email);

        // Assert
        assert_eq!(to_validate_local_part, expected_local_part);
    }

    #[test]
    fn test_fn_get_local_part_if_the_error_is_no_local_part_detected() {
        // Arrange
        let email: String = String::from("@gmail.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::NoLocalPartDetected);

        // Act
        let to_validate_local_part: Result<String, EmailValidationError> =
            Email::get_local_part(&email);

        // Assert
        assert_eq!(to_validate_local_part, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_local_part_if_the_error_is_invalid_starting_or_ending_character() {
        // Arrange
        let email: String = String::from(".Vikenzoespindolaa@gmail.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::InvalidStartEndCharInLocalPart);

        // Act
        let to_validate_local_part: Result<String, EmailValidationError> =
            Email::get_local_part(&email);

        // Assert
        assert_eq!(to_validate_local_part, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_local_part_if_the_error_is_invalid_character() {
        // Arrange
        let email: String = String::from("Vikenzo*espindolaa@gmail.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::InvalidLocalPartCharacter);

        // Act
        let to_validate_local_part: Result<String, EmailValidationError> =
            Email::get_local_part(&email);

        // Assert
        assert_eq!(to_validate_local_part, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_local_part_if_the_error_is_presence_of_special_character_in_a_row() {
        // Arrange
        let email: String = String::from("Vikenzo--espindolaa@gmail.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::SpecialCharInARowInLocalPart);

        // Act
        let to_validate_local_part: Result<String, EmailValidationError> =
            Email::get_local_part(&email);

        // Assert
        assert_eq!(to_validate_local_part, expected_email_validation_error);
    }

    // EMAIL DOMAIN ERRORS TESTS

    #[test]
    fn test_fn_get_email_domain_if_there_is_no_error() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@gmail.com");
        let expected_email_domain: Result<String, EmailValidationError> = Ok(String::from("gmail"));
        // Act
        let to_validate_email_domain: Result<String, EmailValidationError> =
            Email::get_email_domain(&email);
        // Assert
        assert_eq!(to_validate_email_domain, expected_email_domain);
    }

    #[test]
    fn test_fn_get_email_domain_if_the_error_is_no_email_domain_detected() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::NoEmailDomainDetected);
        // Act
        let to_validate_email_domain: Result<String, EmailValidationError> =
            Email::get_email_domain(&email);
        // Assert
        assert_eq!(to_validate_email_domain, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_email_domain_if_the_error_is_invalid_starting_ending_character() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@-gmail.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::InvalidEmailDomainStartEndCharacter);
        // Act
        let to_validate_email_domain: Result<String, EmailValidationError> =
            Email::get_email_domain(&email);
        // Assert
        assert_eq!(to_validate_email_domain, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_email_domain_if_the_error_is_invalid_character() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@gm*ail");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::InvalidEmailDomainCharacter);
        // Act
        let to_validate_email_domain: Result<String, EmailValidationError> =
            Email::get_email_domain(&email);
        // Assert
        assert_eq!(to_validate_email_domain, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_email_domain_if_the_error_is_special_character_in_a_row() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@g--mail.com");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::SpecialCharInARowInEmailDomain);
        // Act
        let to_validate_email_domain: Result<String, EmailValidationError> =
            Email::get_email_domain(&email);
        // Assert
        assert_eq!(to_validate_email_domain, expected_email_validation_error);
    }

    // TOP DOMAIN ERRORS TESTS

    #[test]
    fn test_fn_get_top_domain_if_there_is_no_error() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@gmail.com");
        let expected_top_domain: Result<String, EmailValidationError> = Ok(String::from(".com"));
        // Act
        let to_validate_top_domain: Result<String, EmailValidationError> =
            Email::get_top_domain(&email);
        // Assert
        assert_eq!(to_validate_top_domain, expected_top_domain);
    }

    #[test]
    fn test_fn_get_top_domain_if_the_error_is_no_top_domain_detected() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@.");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::NoTopDomainDetected);
        // Act
        let to_validate_top_domain: Result<String, EmailValidationError> =
            Email::get_top_domain(&email);
        // Assert
        assert_eq!(to_validate_top_domain, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_top_domain_if_the_error_is_invalid_lenght() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@.b");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::TopDomainLengthInvalid);
        // Act
        let to_validate_top_domain: Result<String, EmailValidationError> =
            Email::get_top_domain(&email);
        // Assert
        assert_eq!(to_validate_top_domain, expected_email_validation_error);
    }

    #[test]
    fn test_fn_get_top_domain_if_the_error_is_invalid_character() {
        // Arrange
        let email: String = String::from("Vikenzoespindolaa@.co*m");
        let expected_email_validation_error: Result<String, EmailValidationError> =
            Err(EmailValidationError::InvalidTopDomainCharacter);
        // Act
        let to_validate_top_domain: Result<String, EmailValidationError> =
            Email::get_top_domain(&email);
        // Assert
        assert_eq!(to_validate_top_domain, expected_email_validation_error);
    }

    // CREATE EMAIL VALIDATION ERRORS

    #[test]
    fn test_if_fn_valid_email_construct_works_if_user_input_is_valid() {
        // Arrange

        let email: String = String::from("Vikenzoespindolaa@gmail.com");
        let expected_email: Result<Email, EmailValidationError> = Ok(Email {
            local_part: String::from("Vikenzoespindolaa"),
            email_domain: String::from("gmail"),
            top_domain: String::from(".com"),
        });

        // Act

        let to_validate_email: Result<Email, EmailValidationError> = Email::get_valid_email(email);

        // Assert

        assert_eq!(expected_email, to_validate_email);
    }
}
