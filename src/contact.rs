use std::fmt::Display;

pub struct Contact {
    name: String,
    phone_number: String,
    email: String,
}

impl Contact {
    pub fn new(name: String, number: String, email: String) -> Self {
        Self {
            name,
            phone_number: number,
            email,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_phone_number(&self) -> &String {
        &self.phone_number
    }
    pub fn get_email(&self) -> &String {
        &self.email
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_number(&mut self, number: String) {
        self.phone_number = number;
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email
    }
}
impl Default for Contact {
    fn default() -> Self {
        Self {
            name: Default::default(),
            phone_number: Default::default(),
            email: Default::default(),
        }
    }
}
impl Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let email: String = {
            if self.email.is_empty() {
                "None".to_string()
            } else {
                String::from(self.get_email())
            }
        };
        write!(
            f,
            "Name: {} Phone Number: {} Email: {}",
            self.get_name(),
            self.get_phone_number(),
            email,
        )
    }
}
