use crate::contact::Contact;

pub struct ContactList {
    list: Vec<Contact>,
}

impl ContactList {
    pub const fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add_contact(&mut self, contact: Contact) {
        self.list.push(contact)
    }

    pub fn remove_contact(&mut self, name: String) -> bool {
        let i = self.index_of(&name);
        match i {
            Some(val) => {
                self.list.remove(val);
                true
            }
            None => false,
        }
    }

    pub fn exists(&self, name: &String) -> bool {
        let i = self.index_of(name);
        i.is_some()
    }

    fn index_of(&self, name: &String) -> Option<usize> {
        match self
            .list
            .iter()
            .position(|element| return element.get_name().eq(name))
        {
            Some(index) => return Some(index),
            None => None,
        }
    }

    pub fn modify_contact(
        &mut self,
        contact_name: String,
        new_name: String,
        number: String,
        email: String,
    ) -> bool {
        if self.remove_contact(contact_name) {
            let new_contact = Contact::new(new_name, number, email);
            self.add_contact(new_contact);
            return true;
        }
        false
    }
    pub fn modify_contact_name(&mut self, name: String, new_name: String) -> bool {
        let index = self.index_of(&name);
        match index {
            Some(val) => {
                self.list[val].set_name(new_name);
                true
            }
            None => false,
        }
    }
    pub fn modify_contact_number(&mut self, name: String, number: String) -> bool {
        let index = self.index_of(&name);
        match index {
            Some(val) => {
                self.list[val].set_number(number);
                true
            }
            None => false,
        }
    }
    pub fn modify_contact_email(&mut self, name: String, email: String) -> bool {
        let index = self.index_of(&name);
        match index {
            Some(val) => {
                self.list[val].set_email(email);
                true
            }
            None => false,
        }
    }
    pub fn get_size(&self) -> usize {
        self.list.len()
    }
    pub fn print_contacts(&self) {
        println!("Printing Contacts...");
        for i in 0..self.get_size() {
            println!("CONTACT #{}:\n {}", i, self.list[i])
        }
    }
}
