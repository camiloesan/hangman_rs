use postgres::{Client, NoTls};
use sha2::{Digest, Sha256};

struct User {
    username: String,
    email: String,
    password: String,
}

fn get_client() -> Client {
    let client = Client::connect(
        "postgresql://postgres:postgres@localhost/hangman", 
        NoTls
    );

    match client {
        Ok(client_result) => return client_result,
        Err(e) => panic!("Failed to establish connection: {}", e),
    }
}

fn add_user(username: String, email: String, password: String) -> u64 {
    let mut client = get_client();
    let mut hasher = Sha256::new();
    hasher.update(password);

    let user = User {
        username,
        email,
        password: format!("{:X}", hasher.finalize()),
    };

    let result = client.execute(
        "INSERT INTO \"user\" (username, email, password) VALUES ($1, $2, $3)",
        &[&user.username, &user.email, &user.password],
    );

    match result {
        Ok(rows_affected) => return rows_affected,
        Err(_) => return 0,
    }
}

fn is_auth_valid(username: String, password: String) -> bool {
    let mut client = get_client();
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hashed_pass = format!("{:X}", hasher.finalize());

    let result = client.query("SELECT COUNT(*) FROM \"user\" WHERE username = $1 and password = $2", &[&username, &hashed_pass]);

    match result {
        Ok(value) => {
            if value.len() == 1 {
                return true;
            }
        },
        Err(_) => return false,
    }

    false
}

fn _delete_user_by_username(username: String) -> u64 {
    let mut client = get_client();

    let result = client.execute(
        "DELETE FROM \"user\" WHERE username = ($1)",
        &[&username],
    );

    match result {
        Ok(rows_affected) => return rows_affected,
        Err(_) => return 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::dal::user::*;

    #[test]
    fn add_user_correct() {
        let username = String::from("paulina");
        let email = String::from("pau@gmail.com");
        let password = String::from("bluealex");

        let result = add_user(username.clone(), email, password);
        _delete_user_by_username(username);

        assert_eq!(result, 1);
    }

    #[test]
    fn auth_correct() {
        let username = String::from("paulina");
        let email = String::from("pau@gmail.com");
        let password = String::from("hola");

        let _ = add_user(username.clone(), email, password.clone());
        let result = is_auth_valid(username.clone(), password);
        _delete_user_by_username(username);

        assert!(result);
    }
}