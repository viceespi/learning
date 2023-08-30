#[derive(PartialEq, Eq, Debug, Clone,serde::Serialize, serde::Deserialize)]
pub struct Cpf {
    pub first_nine_digits: Vec<u32>,
    pub last_two_digits: Vec<u32>,
}

#[derive(PartialEq, Eq, Debug)]

pub enum CpfValidationErrors {
    NoUserCPFInputDetected,
    InvalidCPFLenght,
    InvalidPenultimateDigitOfCPF,
    InvalidLastDigitOfCPF,
}

impl Cpf {
    fn get_valid_cpf_penultimate_digit(cpf: &Vec<u32>) -> Result<u32, CpfValidationErrors> {
        let mut validation_number: u32 = 10;
        let mut cpf_iterator_index: usize = 0;
        let mut sum_of_values: u32 = 0;
        while cpf_iterator_index < 9 {
            let cpf_number: &u32 = match cpf.get(cpf_iterator_index) {
                Some(number) => number,
                None => return Err(CpfValidationErrors::NoUserCPFInputDetected),
            };
            sum_of_values += cpf_number * validation_number;
            validation_number -= 1;
            cpf_iterator_index += 1;
        }
        if sum_of_values % 11 < 2 {
            Ok(0)
        } else {
            Ok(11 - (sum_of_values % 11))
        }
    }

    fn get_valid_cpf_last_digit(cpf: &Vec<u32>) -> Result<u32, CpfValidationErrors> {
        let mut validation_number: u32 = 11;
        let mut cpf_iterator_index: usize = 0;
        let mut sum_of_values: u32 = 0;
        while cpf_iterator_index < 10 {
            let cpf_number: &u32 = match cpf.get(cpf_iterator_index) {
                Some(number) => number,
                None => return Err(CpfValidationErrors::NoUserCPFInputDetected),
            };
            sum_of_values += cpf_number * validation_number;
            validation_number -= 1;
            cpf_iterator_index += 1;
        }
        if sum_of_values % 11 < 2 {
            Ok(0)
        } else {
            Ok(11 - (sum_of_values % 11))
        }
    }

    fn get_cpf_first_nine_digits(cpf: &Vec<u32>) -> Result<Vec<u32>, CpfValidationErrors> {
        let mut cpf_nine_digits: Vec<u32> = vec![];
        let mut cpf_iterator_index: usize = 0;
        while cpf_iterator_index < 9 {
            let cpf_number: &u32 = match cpf.get(cpf_iterator_index) {
                Some(number) => number,
                None => return Err(CpfValidationErrors::NoUserCPFInputDetected),
            };
            cpf_nine_digits.push(*cpf_number);
            cpf_iterator_index += 1;
        }
        Ok(cpf_nine_digits)
    }

    fn get_cpf_last_two_digits(cpf: &Vec<u32>) -> Result<Vec<u32>, CpfValidationErrors> {
        let mut cpf_last_digits: Vec<u32> = vec![];
        let mut cpf_iterator_index: usize = 9;
        while cpf_iterator_index < 11 {
            let cpf_number: &u32 = match cpf.get(cpf_iterator_index) {
                Some(number) => number,
                None => return Err(CpfValidationErrors::NoUserCPFInputDetected),
            };
            cpf_last_digits.push(*cpf_number);
            cpf_iterator_index += 1;
        }
        Ok(cpf_last_digits)
    }

    pub fn get_valid_cpf_struct(cpf: Vec<u32>) -> Result<Cpf, CpfValidationErrors> {
        if cpf.is_empty() {
            return Err(CpfValidationErrors::NoUserCPFInputDetected);
        }
        if cpf.len() != 11 {
            return Err(CpfValidationErrors::InvalidCPFLenght);
        }
        let valid_first_last_digit: u32 = match Cpf::get_valid_cpf_penultimate_digit(&cpf) {
            Ok(number) => number,
            Err(error) => return Err(error),
        };
        let valid_last_digit: u32 = match Cpf::get_valid_cpf_last_digit(&cpf) {
            Ok(number) => number,
            Err(error) => return Err(error),
        };
        let mut cpf_iterator_index: usize = 0;
        while cpf_iterator_index < 11 {
            match cpf.get(cpf_iterator_index) {
                Some(number) if cpf_iterator_index == 9 && *number != valid_first_last_digit => {
                    return Err(CpfValidationErrors::InvalidPenultimateDigitOfCPF)
                }
                Some(number) if cpf_iterator_index == 10 && *number != valid_last_digit => {
                    return Err(CpfValidationErrors::InvalidLastDigitOfCPF)
                }
                Some(number) => number,
                None => return Err(CpfValidationErrors::InvalidCPFLenght),
            };
            cpf_iterator_index += 1;
        }
        let user_cpf: Cpf = Cpf {
            first_nine_digits: match Cpf::get_cpf_first_nine_digits(&cpf) {
                Ok(vec_i32) => vec_i32,
                Err(error) => return Err(error),
            },
            last_two_digits: match Cpf::get_cpf_last_two_digits(&cpf) {
                Ok(vec_i32) => vec_i32,
                Err(error) => return Err(error),
            },
        };
        Ok(user_cpf)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TESTS FOR FN GET CPF PENULTIMATE DIGIT

    #[test]
    fn test_if_fn_get_cpf_penultimate_digit_works_if_cpf_is_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 7, 6];
        let first_last_digit: Result<u32, CpfValidationErrors> = Ok(cpf[9]);
        let to_validate_first_last_digit: Result<u32, CpfValidationErrors> =
            Cpf::get_valid_cpf_penultimate_digit(&cpf);
        assert_eq!(first_last_digit, to_validate_first_last_digit);
    }

    #[test]
    fn test_if_fn_get_cpf_penultimate_digit_returns_correct_error_when_there_is_no_user_input() {
        let cpf: Vec<u32> = vec![];
        let expected_error: Result<u32, CpfValidationErrors> =
            Err(CpfValidationErrors::NoUserCPFInputDetected);
        let to_validate_error: Result<u32, CpfValidationErrors> =
            Cpf::get_valid_cpf_penultimate_digit(&cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    // TESTS FOR FN GET CPF LAST DIGIT

    #[test]
    fn test_if_fn_get_cpf_last_digit_works_if_cpf_is_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 7, 6];
        let first_last_digit: Result<u32, CpfValidationErrors> = Ok(cpf[10]);
        let to_validate_first_last_digit: Result<u32, CpfValidationErrors> =
            Cpf::get_valid_cpf_last_digit(&cpf);
        assert_eq!(first_last_digit, to_validate_first_last_digit);
    }

    #[test]
    fn test_if_fn_get_cpf_last_digit_returns_correct_error_when_there_is_no_user_input() {
        let cpf: Vec<u32> = vec![];
        let expected_error: Result<u32, CpfValidationErrors> =
            Err(CpfValidationErrors::NoUserCPFInputDetected);
        let to_validate_error: Result<u32, CpfValidationErrors> =
            Cpf::get_valid_cpf_penultimate_digit(&cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    // TESTS FOR FN GET CPF FIRST NINE DIGITS

    #[test]
    fn test_if_fn_get_cpf_first_nine_digits_works_if_cpf_is_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 7, 6];
        let cpf_first_nine_digits: Result<Vec<u32>, CpfValidationErrors> =
            Ok(vec![0, 1, 8, 8, 6, 3, 8, 7, 1]);
        let to_validate_nine_digits: Result<Vec<u32>, CpfValidationErrors> =
            Cpf::get_cpf_first_nine_digits(&cpf);
        assert_eq!(cpf_first_nine_digits, to_validate_nine_digits);
    }

    #[test]
    fn test_if_fn_get_cpf_first_nine_digits_returns_correct_error_when_there_is_no_user_input() {
        let cpf: Vec<u32> = vec![];
        let expected_error: Result<u32, CpfValidationErrors> =
            Err(CpfValidationErrors::NoUserCPFInputDetected);
        let to_validate_error: Result<u32, CpfValidationErrors> =
            Cpf::get_valid_cpf_penultimate_digit(&cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    // TESTS FOR FN GET CPF LAST TWO DIGITS

    #[test]
    fn test_if_fn_get_cpf_last_two_digits_works_if_cpf_is_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 7, 6];
        let last_two_digits: Result<Vec<u32>, CpfValidationErrors> = Ok(vec![7, 6]);
        let to_validate_last_two_digits: Result<Vec<u32>, CpfValidationErrors> =
            Cpf::get_cpf_last_two_digits(&cpf);
        assert_eq!(last_two_digits, to_validate_last_two_digits);
    }

    #[test]
    fn test_if_fn_get_cpf_last_two_digit_returns_correct_error_when_there_is_no_user_input() {
        let cpf: Vec<u32> = vec![];
        let expected_error: Result<u32, CpfValidationErrors> =
            Err(CpfValidationErrors::NoUserCPFInputDetected);
        let to_validate_error: Result<u32, CpfValidationErrors> =
            Cpf::get_valid_cpf_penultimate_digit(&cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    // TESTS FOR FN GET VALID CPF STRUCT

    #[test]
    fn test_if_fn_get_valid_cpf_struct_works_if_cpf_is_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 7, 6];
        let expected_cpf: Result<Cpf, CpfValidationErrors> = Ok(Cpf {
            first_nine_digits: vec![0, 1, 8, 8, 6, 3, 8, 7, 1],
            last_two_digits: vec![7, 6],
        });
        let to_validate_cpf_struct: Result<Cpf, CpfValidationErrors> =
            Cpf::get_valid_cpf_struct(cpf);
        assert_eq!(expected_cpf, to_validate_cpf_struct);
    }
    #[test]
    fn test_if_fn_get_valid_cpf_struct_returns_correct_error_wen_cpf_lenght_is_invalid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 7];
        let expected_error: Result<Cpf, CpfValidationErrors> =
            Err(CpfValidationErrors::InvalidCPFLenght);
        let to_validate_error: Result<Cpf, CpfValidationErrors> = Cpf::get_valid_cpf_struct(cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    #[test]
    fn test_if_fn_get_valid_cpf_struct_returns_correct_error_wen_there_is_no_user_input() {
        let cpf: Vec<u32> = vec![];
        let expected_error: Result<Cpf, CpfValidationErrors> =
            Err(CpfValidationErrors::NoUserCPFInputDetected);
        let to_validate_error: Result<Cpf, CpfValidationErrors> = Cpf::get_valid_cpf_struct(cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    #[test]
    fn test_if_fn_get_valid_cpf_struct_returns_correct_error_wen_penultimate_digit_isnt_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 8, 6];
        let expected_error: Result<Cpf, CpfValidationErrors> =
            Err(CpfValidationErrors::InvalidPenultimateDigitOfCPF);
        let to_validate_error: Result<Cpf, CpfValidationErrors> = Cpf::get_valid_cpf_struct(cpf);
        assert_eq!(expected_error, to_validate_error);
    }

    #[test]
    fn test_if_fn_get_valid_cpf_struct_returns_correct_error_wen_last_digit_isnt_valid() {
        let cpf: Vec<u32> = vec![0, 1, 8, 8, 6, 3, 8, 7, 1, 8, 6];
        let expected_error: Result<Cpf, CpfValidationErrors> =
            Err(CpfValidationErrors::InvalidCPFLenght);
        let to_validate_error: Result<Cpf, CpfValidationErrors> = Cpf::get_valid_cpf_struct(cpf);
        assert_eq!(expected_error, to_validate_error);
    }
}
