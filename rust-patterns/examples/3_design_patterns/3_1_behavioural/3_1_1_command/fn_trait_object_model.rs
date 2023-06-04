type Migration<'a> = Box<dyn Fn() -> &'a str>;

pub struct Schema<'a> {
    executes: Vec<Migration<'a>>,
    rollbacks: Vec<Migration<'a>>,
}

impl<'a> Schema<'a> {
    pub fn new() -> Self {
        Self {
            executes: vec![],
            rollbacks: vec![],
        }
    }
    pub fn add_migration<E, R>(&mut self, execute: E, rollback: R)
    where
        E: Fn() -> &'a str + 'static,
        R: Fn() -> &'a str + 'static,
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));
    }
    pub fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }
    pub fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

pub fn add_field() -> &'static str {
    "add field"
}

pub fn remove_field() -> &'static str {
    "remove field"
}
