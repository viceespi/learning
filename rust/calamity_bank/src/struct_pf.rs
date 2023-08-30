use crate::struct_adress::Adress;
use crate::struct_cpf::Cpf;
use crate::struct_date_of_birth_v2::Dob;
use crate::struct_email::Email;
use crate::struct_name_v2::Name;
use crate::struct_telephone::Telephone;
#[derive(Clone,serde::Serialize, serde::Deserialize, Debug)]
pub struct PF {
    pub name: Name,
    pub cpf: Cpf,
    pub date_of_birth: Dob,
    pub email: Email,
    pub telephone: Telephone,
    pub adress: Adress,
}

impl PF {
    pub fn create_new_pf(
        user_name: Name,
        user_cpf: Cpf,
        user_date_of_birth: Dob,
        user_email: Email,
        user_telephone: Telephone,
        user_adress: Adress,
    ) -> Self {
        let user_pf_registration: PF = PF {
            name: user_name,
            cpf: user_cpf,
            date_of_birth: user_date_of_birth,
            email: user_email,
            telephone: user_telephone,
            adress: user_adress,
        };
        user_pf_registration
    }
}
