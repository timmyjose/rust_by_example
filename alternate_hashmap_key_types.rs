use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn main() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.evreman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Evreman",
        email: "john@evreman.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.evreman", "password123");
    try_logon(&accounts, "j.evreman", "p1assword123");
    try_logon(&accounts, "j.evremann", "password123");
}

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon..");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Name: {}, email: {}", account_info.name, account_info.email)
        }
        _ => println!("account not found in system"),
    }
}
