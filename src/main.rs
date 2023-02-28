#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation};
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a User struct to hold user data
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    password: String,
}

// Define a UserDatabase struct to store user data
struct UserDatabase {
    users: HashMap<i32, User>,
    next_id: i32,
}

impl UserDatabase {
    fn new() -> UserDatabase {
        UserDatabase {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.insert(self.next_id, user);
        self.next_id += 1;
    }

    fn get_user_by_username(&self, username: &str) -> Option<&User> {
        for (_, user) in &self.users {
            if user.username == username {
                return Some(user);
            }
        }
        None
    }

    fn get_user_by_id(&self, id: i32) -> Option<&User> {
        self.users.get(&id)
    }
}

// Define a Token struct to hold token data
#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String,
}

// Define a LoginRequest struct to hold login request data
#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// Define a LoginResponse struct to hold login response data
#[derive(Debug, Serialize)]
struct LoginResponse {
    id: i32,
    username: String,
    token: String,
}

// Define a CustomError struct to hold error data
#[derive(Debug, Serialize)]
struct CustomError {
    error: String,
}

// Define a route to handle login requests and return a token if the login is successful
#[post("/login", format = "json", data = "<login_request>")]
fn login(user_database: rocket::State<UserDatabase>, login_request: Json<LoginRequest>) -> Result<Json<LoginResponse>, status::Custom<Json<CustomError>>> {
    let user = user_database.get_user_by_username(&login_request.username);
    if let Some(user) = user {
        if verify(&login_request.password, &user.password).unwrap() {
            let token = encode(&Header::default(), &user.id, "secret".as_ref(), Algorithm::HS256).unwrap();
            let login_response = LoginResponse { id: user.id, username: user.username.clone(), token };
            Ok(Json(login_response))
        } else {
            Err(status::Custom(Status::Unauthorized, Json(CustomError { error: "Invalid username or password".to_string() })))
        }
    } else {
