#[derive(PartialEq, Eq, Debug,Clone,serde::Serialize, serde::Deserialize)]
pub struct Dob {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub enum DOBValidationError {
    InvalidAge,
    InvalidMonth,
    InvalidDay,
    NoUserInputDetected,
}

impl Dob {
    fn get_valid_year(user_year_of_birth: i32) -> Result<i32, DOBValidationError> {
        let current_year: i32 = 2023;
        if current_year - user_year_of_birth < 18 || current_year - user_year_of_birth > 115 {
            return Err(DOBValidationError::InvalidAge);
        }
        Ok(user_year_of_birth)
    }

    fn get_valid_month(user_month_of_birth: i32) -> Result<i32, DOBValidationError> {
        if user_month_of_birth < 1 || user_month_of_birth > 12 {
            return Err(DOBValidationError::InvalidMonth);
        }
        Ok(user_month_of_birth)
    }

    fn check_if_dob_year_is_leap_year(user_year_of_birth: i32) -> bool {
        let mut dob_year_is_leap_year: bool = false;
        if (user_year_of_birth % 4) == 0 {
            if (user_year_of_birth % 100) == 0 {
                if (user_year_of_birth % 400) == 0 {
                    dob_year_is_leap_year = true;
                }
            } else {
                dob_year_is_leap_year = true;
            }
        }
        dob_year_is_leap_year
    }

    fn get_max_days_of_month(user_year_of_birth: i32, user_month_of_birth: i32) -> i32 {
        let is_leap_year: bool = Dob::check_if_dob_year_is_leap_year(user_year_of_birth);
        let is_month_even: bool = user_month_of_birth % 2 == 0;
        match (user_month_of_birth, is_leap_year, is_month_even) {
            (2, true, _) => 29,
            (2, false, _) => 28,
            (month, _, true) if month <= 7 => 30,
            (month, _, false) if month <= 7 => 31,
            (month, _, true) if month >= 8 => 31,
            _ => 30,
        }
    }

    fn get_valid_day(
        user_year_of_birth: i32,
        user_month_of_birth: i32,
        user_day_of_birth: i32,
    ) -> Result<i32, DOBValidationError> {
        let max_month_days: i32 =
            Dob::get_max_days_of_month(user_year_of_birth, user_month_of_birth);
        if user_day_of_birth < 1 {
            return Err(DOBValidationError::InvalidDay);
        }
        if user_day_of_birth > max_month_days {
            return Err(DOBValidationError::InvalidDay);
        }
        Ok(user_day_of_birth)
    }

    pub fn get_valid_date_of_birth(
        user_year_of_birth: i32,
        user_month_of_birth: i32,
        user_day_of_birth: i32,
    ) -> Result<Dob, DOBValidationError> {
        let user_date_of_birth = Dob {
            year: match Dob::get_valid_year(user_year_of_birth) {
                Ok(year) => year,
                Err(error) => return Err(error),
            },
            month: match Dob::get_valid_month(user_month_of_birth) {
                Ok(month) => month,
                Err(error) => return Err(error),
            },
            day: match Dob::get_valid_day(
                user_year_of_birth,
                user_month_of_birth,
                user_day_of_birth,
            ) {
                Ok(day) => day,
                Err(error) => return Err(error),
            },
        };
        Ok(user_date_of_birth)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // YEAR VALIDATION TESTS

    #[test]
    fn test_fn_get_year_if_year_is_valid() {
        let user_year_of_birth: i32 = 1998;
        let expected_year: Result<i32, DOBValidationError> = Ok(1998);
        let to_check_year: Result<i32, DOBValidationError> =
            Dob::get_valid_year(user_year_of_birth);
        assert_eq!(to_check_year, expected_year);
    }

    #[test]
    fn test_fn_get_year_if_age_is_invalid_and_age_is_bigger_than_expected() {
        let user_year_of_birth: i32 = 1900;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidAge);
        let to_check_year: Result<i32, DOBValidationError> =
            Dob::get_valid_year(user_year_of_birth);
        assert_eq!(to_check_year, expected_dob_error);
    }

    #[test]
    fn test_fn_get_year_if_age_is_invalid_and_age_is_smaller_than_expected() {
        let user_year_of_birth: i32 = 2015;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidAge);
        let to_check_year: Result<i32, DOBValidationError> =
            Dob::get_valid_year(user_year_of_birth);
        assert_eq!(to_check_year, expected_dob_error);
    }

    // MONTH VALIDATION TESTS

    #[test]
    fn test_fn_get_month_if_month_is_valid() {
        let user_month_of_birth: i32 = 9;
        let expected_month: Result<i32, DOBValidationError> = Ok(9);
        let to_check_month: Result<i32, DOBValidationError> =
            Dob::get_valid_month(user_month_of_birth);
        assert_eq!(to_check_month, expected_month);
    }

    #[test]
    fn test_fn_get_month_if_month_is_invalid_because_it_is_bigger_than_expected() {
        let user_month_of_birth: i32 = 19;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidMonth);
        let to_check_month: Result<i32, DOBValidationError> =
            Dob::get_valid_month(user_month_of_birth);
        assert_eq!(to_check_month, expected_dob_error);
    }

    #[test]
    fn test_fn_get_month_if_month_is_invalid_because_it_is_smaller_than_expected() {
        // Arrange

        let user_month_of_birth: i32 = 0;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidMonth);
        let to_check_month: Result<i32, DOBValidationError> =
            Dob::get_valid_month(user_month_of_birth);
        assert_eq!(to_check_month, expected_dob_error);
    }

    // DAY VALIDATION TESTS

    #[test]
    fn test_fn_get_day_if_day_is_valid() {
        let user_year_of_birth: i32 = 1998;
        let user_month_of_birth: i32 = 9;
        let user_day_of_birth: i32 = 29;
        let expected_day: Result<i32, DOBValidationError> = Ok(29);
        let to_check_day: Result<i32, DOBValidationError> =
            Dob::get_valid_day(user_year_of_birth, user_month_of_birth, user_day_of_birth);
        assert_eq!(to_check_day, expected_day);
    }

    #[test]
    fn test_fn_get_day_if_day_is_invalid_because_it_is_29_2_in_a_non_leap_year() {
        let user_year_of_birth: i32 = 1998;
        let user_month_of_birth: i32 = 2;
        let user_day_of_birth: i32 = 29;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidDay);
        let to_check_day: Result<i32, DOBValidationError> =
            Dob::get_valid_day(user_year_of_birth, user_month_of_birth, user_day_of_birth);
        assert_eq!(to_check_day, expected_dob_error);
    }

    #[test]
    fn test_fn_get_day_if_day_is_invalid_because_it_is_31_on_a_even_month_between_january_and_july()
    {
        let user_year_of_birth: i32 = 1998;
        let user_month_of_birth: i32 = 4;
        let user_day_of_birth: i32 = 31;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidDay);
        let to_check_day: Result<i32, DOBValidationError> =
            Dob::get_valid_day(user_year_of_birth, user_month_of_birth, user_day_of_birth);
        assert_eq!(to_check_day, expected_dob_error);
    }

    #[test]
    fn test_fn_get_day_if_day_is_invalid_because_it_is_31_on_a_odd_month_between_august_and_december(
    ) {
        let user_year_of_birth: i32 = 1998;
        let user_month_of_birth: i32 = 9;
        let user_day_of_birth: i32 = 31;
        let expected_dob_error: Result<i32, DOBValidationError> =
            Err(DOBValidationError::InvalidDay);
        let to_check_day: Result<i32, DOBValidationError> =
            Dob::get_valid_day(user_year_of_birth, user_month_of_birth, user_day_of_birth);
        assert_eq!(to_check_day, expected_dob_error);
    }
}
