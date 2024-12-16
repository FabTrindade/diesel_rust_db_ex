extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io;

// Importa o módulo `schema` que contém as definições da tabela
mod schema;
use self::schema::users;
use self::schema::users::dsl::*;

#[derive(Queryable)]
struct User {
    id: i32,
    user_name: String,
    email: String,
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
    user_name: &'a str,
    email: &'a str,
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    println!("Type 'c' to insert new user. Other key to continue to list users");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "c" {
        add_user(&mut connection);
    } 
    
    list_users(&mut connection);    
}

fn list_users(connection: &mut PgConnection) {
    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} users...\n", results.len());
    println!("ID \t User name \t email");
    println!("------------------------------");
    for user in results {
        println!("{}\t {}\t {}", user.id, user.user_name, user.email);
    }
    println!("------------------------------\n");
}

fn add_user(connection: &mut PgConnection) {
    let mut user_name_ = String::new();
    let mut email_ = String::new();

    println!("Digite o nome do usuário:");
    io::stdin().read_line(&mut user_name_).expect("Failed to read line");

    println!("Digite o email do usuário:");
    io::stdin().read_line(&mut email_).expect("Failed to read line");

    let new_user = NewUser {
        user_name: user_name_.trim(),
        email: email_.trim(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    println!("Usuário {} adicionado com sucesso!", new_user.user_name);
}
