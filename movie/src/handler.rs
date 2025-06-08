use std::error::Error;

use crate::services::{get_users, login_success, logout};

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("Username: {username}");
    if let Some(user) = get_users()
        .iter()
        .find(|u| u.username.eq_ignore_ascii_case(username))
    {
        println!("Please enter the password:");
        match rpassword::read_password() {
            Ok(password) => {
                if user.password == password {
                    login_success(&user.role)?;
                    println!("Log in successfully");
                } else {
                    println!("Incorrent password.");
                }
            }
            Err(_) => {
                println!("Failed to read password.");
            }
        }
    } else {
        println!("user to read password");
    }
    Ok(())
}

pub fn handle_logout() {
    logout();
    println!("Logged out successfully");
}