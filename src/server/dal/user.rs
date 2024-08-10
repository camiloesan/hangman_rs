use postgres::{Client, NoTls};
use sha2::{Digest, Sha256};

pub struct User {
    user_id: Option<i32>,
    username: String,
    email: String,
    password: Option<String>,
}

impl User {
    pub fn get_user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }
}

fn get_client() -> Client {
    let client = Client::connect(
        "postgresql://camilo:postgres@localhost/hangman", 
        NoTls
    );

    match client {
        Ok(client_result) => return client_result,
        Err(e) => panic!("Failed to establish connection: {}", e),
    }
}

pub fn add_user(username: String, email: String, password: String) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(password);

    let user = User {
        user_id: None,
        username,
        email,
        password: Some(format!("{:X}", hasher.finalize())),
    };

    let mut client = get_client();

    let result = client.execute(
        "INSERT INTO \"user\" (username, email, password) VALUES ($1, $2, $3)",
        &[&user.username, &user.email, &user.password.unwrap()],
    );

    match result {
        Ok(rows_affected) => rows_affected,
        Err(_) => return 0,
    }
}

pub fn is_auth_valid(username: String, password: String) -> bool {
    let mut client = get_client();
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hashed_pass = format!("{:X}", hasher.finalize());

    let result = client.query(
        "SELECT COUNT(*) FROM \"user\" WHERE username = $1 and password = $2",
        &[&username, &hashed_pass],
    );

    match result {
        Ok(value) => {
            if value.len() == 1 {
                return true;
            }
        }
        Err(_) => return false,
    }

    false
}

pub fn get_user_by_username(username: String) -> User {
    let mut client = get_client();

    let result = client.query(
        "SELECT user_id, username, email FROM \"user\" WHERE username = $1",
        &[&username],
    );

    match result {
        Ok(rows) => {
            let row = rows.get(0).expect("Failed to get user");
            return User {
                user_id: row.get(0),
                username: row.get(1),
                email: row.get(2),
                password: None,
            };
        }
        Err(_) => {
            return User {
                user_id: None,
                username: String::from(""),
                email: String::from(""),
                password: None,
            }
        }
    }
}

pub fn delete_user_by_id(id: i32) -> u64 {
    let mut client = get_client();

    let result = client.execute(
        "DELETE FROM \"user\" WHERE user_id = ($1)", 
        &[&id]
    );

    match result {
        Ok(rows_affected) => return rows_affected,
        Err(_) => return 0,
    }
}

pub fn modify_user_by_id(id: i32, username: String, email: String) -> u64 {
    let mut client = get_client();

    let result = client.execute(
        "UPDATE \"user\" SET username = $1, email = $2 WHERE user_id = $3",
        &[&username, &email, &id],
    );

    match result {
        Ok(rows_affected) => return rows_affected,
        Err(_) => return 0,
    }
}

//modify the user password with 2 factor auth todo

#[cfg(test)]
mod tests {
    use crate::dal::user::*;

    #[test]
    fn add_user_correct() {
        let username = String::from("paulina");
        let email = String::from("pau@gmail.com");
        let password = String::from("bluealex");

        let result = add_user(username.clone(), email, password);
        let user = get_user_by_username(username.clone());
        delete_user_by_id(user.user_id.unwrap());

        assert_eq!(result, 1);
    }

    #[test]
    fn auth_correct() {
        let username = String::from("paulina");
        let email = String::from("pau@gmail.com");
        let password = String::from("hola");

        let _ = add_user(username.clone(), email, password.clone());
        let result = is_auth_valid(username.clone(), password);
        let user = get_user_by_username(username.clone());
        delete_user_by_id(user.user_id.unwrap());

        assert!(result);
    }
}
