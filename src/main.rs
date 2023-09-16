use std::io::stdin;

mod login;
mod marks;

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let is_debug_run_environment_var = std::env::var("BAKALARIUM_DEBUG");
    let is_debug_run = match is_debug_run_environment_var {
        Ok(var) => match var.as_str() {
            "1" => true,
            _ => false
        }
        Err(_) => false
    };

    print_logo!();
    println!("--> Welcome to Bakalarium! You are using version {VERSION}!");
    if is_debug_run {
        println!("--> Running in debug mode!");
    };

    let mut buffer = String::new();
    println!("Enter your username: ");
    stdin().read_line(&mut buffer).unwrap();
    let username = buffer.trim().to_owned().clone();
    buffer.clear();
    println!("Enter your password: ");
    stdin().read_line(&mut buffer).unwrap();
    let password = buffer.trim().to_owned().clone();
    buffer.clear();
    println!("Enter your url: ");
    stdin().read_line(&mut buffer).unwrap();
    let url = buffer.trim().to_owned().clone();
    drop(buffer);

    println!("--> Attempting to login...");
    let login = login::login(&username, 
        &password.to_owned(), 
        &url.parse().unwrap(),
    is_debug_run);

    if is_debug_run {
        println!("Return of the login() function: \n{login:#?}");
    }


}

#[macro_export]
macro_rules! print_logo {
    () => {
        println!(r" _           _         _            _                 
| |         | |       | |          (_)                
| |__   __ _| | ____ _| | __ _ _ __ _ _   _ _ __ ___  
| '_ \ / _` | |/ / _` | |/ _` | '__| | | | | '_ ` _ \ 
| |_) | (_| |   < (_| | | (_| | |  | | |_| | | | | | |
|_.__/ \__,_|_|\_\__,_|_|\__,_|_|  |_|\__,_|_| |_| |_|
                                                       
                                                              ");
    };
}