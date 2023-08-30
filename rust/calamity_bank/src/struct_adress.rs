#[derive(PartialEq, Eq, Debug,Clone,serde::Serialize, serde::Deserialize)]
pub struct Adress {
    pub adress_number: String,
    pub postal_code: String,
}

#[derive(PartialEq, Eq, Debug)]

pub enum AdressValidationErrors {
    NoPostalCodeInputDetected,
    PostalCodeInvalidCharacter,
    PostalCodeInvalidLenght,
}

impl Adress {
    fn validate_postal_code(user_postal_code: String) -> Result<String, AdressValidationErrors> {
        let check_postal_code: String = user_postal_code.trim().to_string();
        if check_postal_code == " " {
            return Err(AdressValidationErrors::NoPostalCodeInputDetected);
        }
        if check_postal_code.len() != 8 {
            return Err(AdressValidationErrors::PostalCodeInvalidLenght);
        }
        for character in check_postal_code.chars() {
            if !character.is_numeric() {
                return Err(AdressValidationErrors::PostalCodeInvalidCharacter);
            }
        }
        Ok(check_postal_code)
    }

    pub fn create_user_adress(
        user_adress_number: String,
        user_postal_code: String,
    ) -> Result<Adress, AdressValidationErrors> {
        let user_adress: Adress = Adress {
            adress_number: user_adress_number,
            postal_code: match Adress::validate_postal_code(user_postal_code) {
                Ok(string) => string,
                Err(error) => return Err(error),
            },
        };
        Ok(user_adress)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // POSTAL CODE VALIDATION ERRORS
    fn check_if_fn_to_validate_postal_code_works_correctly_if_the_adress_has_a_valid_postal_code() {
        // Arrange

        let user_postal_code: String = String::from("15505325");
        let expected_postal_code: Result<String, AdressValidationErrors> =
            Ok(String::from("15505325"));

        // Act
        let to_validate_postal_code: Result<String, AdressValidationErrors> =
            Adress::validate_postal_code(user_postal_code);

        // Assert

        assert_eq!(expected_postal_code, to_validate_postal_code);
    }

    // ADRESS CREATION FN TESTS

    #[test]
    fn check_if_fn_to_create_user_adress_is_working() {
        // Arrange

        let user_street: String = String::from("Rua Dircinho Longo");
        let user_adress_number: String = String::from("6307");
        let user_city: String = String::from("Votuporanga");
        let user_state: String = String::from("São Paulo");
        let user_country: String = String::from("Brazil");
        let user_postal_code: String = String::from("15505325");

        let expected_user_adress: Result<Adress, AdressValidationErrors> = Ok(Adress {
            adress_number: String::from("6307"),
            postal_code: String::from("15505325"),
        });

        // Act

        let to_validate_user_adress: Result<Adress, AdressValidationErrors> =
            Adress::create_user_adress(user_adress_number, user_postal_code);

        // Assert

        assert_eq!(expected_user_adress, to_validate_user_adress);
    }
}
