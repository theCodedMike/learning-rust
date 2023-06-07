pub struct Database {
    pub connection_string: String,
    pub timeout: u32,
    pub pool_size: u32,
}

pub fn print_database(database: &Database) {
    println!("Connection string: {}", database.connection_string);
    println!("Timeout: {}", database.timeout);
    println!("Pool size: {}", database.pool_size);
    println!();
}
