//extern crate diesel;

#[macro_use]
extern crate diesel_migrations;
embed_migrations!("../migrations");

fn main() {
    let operation = std::env::args().nth(1).expect("no pattern given");
    let project = std::env::args().nth(2).expect("no path given");
    println!("Quickset v0.0.1");
    println!("Operation: {operation} - Project: {project}", operation = operation, project = project);
}
