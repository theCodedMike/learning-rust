// Database is now composed of three structs - ConnectionString, Timeout and PoolSize.
// Let's decompose it into smaller structs
#[derive(Debug, Clone)]
pub struct ConnectionString(pub String);

#[derive(Debug, Clone, Copy)]
pub struct Timeout(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct PoolSize(pub u32);

// We then compose these smaller structs back into `Database`
pub struct Database {
    pub connection_string: ConnectionString,
    pub timeout: Timeout,
    pub pool_size: PoolSize,
}

// print_database can then take ConnectionString, Timeout and Poolsize struct instead
pub fn print_database(connection_str: ConnectionString, timeout: Timeout, pool_size: PoolSize) {
    println!("Connection string: {:?}", connection_str);
    println!("Timeout: {:?}", timeout);
    println!("Pool size: {:?}", pool_size);
    println!();
}
