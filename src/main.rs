mod contact;
mod contact_list;
use std::{
    io::{self, Write},
    process,
};

use contact::Contact;
use contact_list::ContactList;

struct State {
    list: ContactList,
}

impl State {
    pub fn new() -> Self {
        Self {
            list: ContactList::new(),
        }
    }
    pub fn get_contacts(&self) -> &ContactList {
        &self.list
    }
}
fn main() {
    let quit = false;
    print_options();

    let mut state = State::new();

    while !quit {
        print!("Select Option: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: i32 = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };
        match input {
            0 => exit(),
            1 => print_contacts(&state),
            2 => {
                let name = read_name();
                println!("* Enter phone number");
                print!("Phone Number: ");
                io::stdout().flush().unwrap();
                let mut number = String::new();
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read input");
                println!("* Enter email: ");
                print!("Email: ");
                io::stdout().flush().unwrap();
                let mut email = String::new();
                io::stdin()
                    .read_line(&mut email)
                    .expect("Failed to read input");
                add_contact(&mut state, name, number, email);
                print_options();
            }
            3 => {
                let name = read_name();
                remove_contact(&mut state, name);
                print_options();
            }
            4 => {
                let name = read_name();
                if state.list.exists(&name) {
                    println!("* Select the field that you want to modify");
                    println!("Enter:\n1. - To modify the name\n2. - To modify the phone number\n3. - To modify the email address\n4. - To modify all of them\n5. - Any other number to go back");
                    let option: u8 = loop {
                        let mut input = String::new();
                        print!("Field: ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input");
                        let input: u8 = match input.trim().parse() {
                            Ok(val) => val,
                            Err(_) => {
                                println!("Enter a valid number");
                                continue;
                            }
                        };
                        break input;
                    };
                    match option {
                        1 => {
                            let new_name = read_name();
                            modify_contact_name(&mut state, name, new_name);
                        }
                        2 => {
                            println!("* New phone number");
                            print!("Phone Number: ");
                            io::stdout().flush().unwrap();
                            let mut number = String::new();
                            io::stdin()
                                .read_line(&mut number)
                                .expect("Failed to read input");
                            modify_contact_number(&mut state, name, number);
                        }
                        3 => {
                            println!("* New email address");
                            print!("Email Address: ");
                            io::stdout().flush().unwrap();
                            let mut email = String::new();
                            io::stdin()
                                .read_line(&mut email)
                                .expect("Failed to read input");
                            modify_contact_email(&mut state, name, email);
                        }
                        4 => {
                            let new_name = read_name();
                            println!("* New phone number");
                            print!("Phone Number: ");
                            io::stdout().flush().unwrap();
                            let mut number = String::new();
                            io::stdin()
                                .read_line(&mut number)
                                .expect("Failed to read input");

                            println!("* New email address");
                            print!("Email Address: ");
                            io::stdout().flush().unwrap();
                            let mut email = String::new();
                            io::stdin()
                                .read_line(&mut email)
                                .expect("Failed to read input");
                            modify_contact(&mut state, name, new_name, number, email);
                        }
                        _ => (),
                    }
                } else {
                    println!("Contact: {} does not exists.", name.trim());
                }
                print_options();
            }
            5 => {
                let contact_name = read_name();
                check_contact(&mut state, contact_name);
            }
            _ => (),
        }
    }
}

fn read_name() -> String {
    println!("* Enter contact name: ");
    print!("Name: ");
    io::stdout().flush().unwrap();
    let mut new_name = String::new();
    io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read input");
    new_name
}
fn check_contact(state: &mut State, contact: String) {
    let text = String::from(&contact);

    if state.list.exists(&contact) {
        println!("{} is included in your contact list!", text.trim());
    } else {
        println!("{} does not exists in your contact list.", text.trim());
    }
}
fn modify_contact(state: &mut State, contact: String, name: String, number: String, email: String) {
    let text = String::from(&name);
    let bl = state.list.modify_contact(contact, name, number, email);
    if bl {
        println!("Your contact was successfully modified!");
    } else {
        println!("Could not modify the contact: {}", text);
    }
}
fn modify_contact_email(state: &mut State, contact: String, number: String) {
    let name_copy = String::from(&contact);
    let bl = state.list.modify_contact_email(contact, number);
    if bl {
        println!("Your contact was successfully modified!");
    } else {
        println!("Could not modify the contact: {}", name_copy);
    }
}
fn modify_contact_number(state: &mut State, contact: String, number: String) {
    let name_copy = String::from(&contact);
    let bl = state.list.modify_contact_number(contact, number);
    if bl {
        println!("Your contact was successfully modified!");
    } else {
        println!("Could not modify the contact: {}", name_copy);
    }
}
fn modify_contact_name(state: &mut State, contact: String, name: String) {
    let name_copy = String::from(&contact);
    let bl = state.list.modify_contact_name(contact, name);
    if bl {
        println!("Your contact was successfully modified!");
    } else {
        println!("Could not modify the contact: {}", name_copy);
    }
}
fn remove_contact(state: &mut State, name: String) {
    let text = String::from(&name);
    let removed = state.list.remove_contact(name);
    if removed {
        println!(
            "The contact: {} was succesffully removed from your contact list!",
            text.as_str().trim()
        );
    } else {
        println!(
            "Could not remove the contact: {}, as it does not exists.",
            text.as_str().trim()
        );
    }
}
fn add_contact(state: &mut State, name: String, number: String, email: String) {
    let text = String::from(&name);
    let contact = Contact::new(name, number, email);
    state.list.add_contact(contact);
    println!(
        "The contact: {} was successfully added to your contact list!",
        text.as_str()
    );
}
fn print_contacts(state: &State) {
    if state.list.get_size() == 0 {
        println!("Your contact list is currently empty.");
    } else {
        state.list.print_contacts();
        print_options();
    }
}
fn exit() {
    println!("Shutting down...");
    process::exit(1);
}

fn print_options() {
    println!("Available Options: \n Press:\n 0 - To Quit \n 1 - To print available contacts \n 2 - To add a contact to the list \n 3 - To remove a contact from the list \n 4 - To modify a contact from the list \n 5 - To check if a contact exists");
}
