use std::collections::HashSet;

/// ```json
/// {
///     "name": "Jane Doe",
///     "dob": "2002-02-24",
///     [...]
///     "customer_id": 1048576332,
/// }
/// ```
/// ```json
/// {
///     "customer_id": 1048576332,
///     "accounts": [
///         {
///             "account_id": 2121,
///             "account_type: "savings",
///             "joint_customer_ids": [],
///             [...]
///         },
///         {
///             "account_id": 2122,
///             "account_type: "checking",
///             "joint_customer_ids": [1048576333],
///             [...]
///         }
///     ]
/// }
/// ```

pub struct Account {
    account_id: u32,
    account_type: String,
    // other fields omitted
}

pub trait CustomerId {
    fn get_customer_id(&self) -> u64;
}

pub struct CreditRecord {
    customer_id: u64,
    name: String,
    dob: String,
    // other fields omitted
}

impl CustomerId for CreditRecord {
    fn get_customer_id(&self) -> u64 {
        self.customer_id
    }
}

pub struct AccountRecord {
    customer_id: u64,
    accounts: Vec<Account>,
}

impl CustomerId for AccountRecord {
    fn get_customer_id(&self) -> u64 {
        self.customer_id
    }
}

// static polymorphism: only one type, but each function call can choose it
fn unique_ids_set<R: CustomerId>(records: &[R]) -> HashSet<u64> {
    records.iter().map(|r| r.get_customer_id()).collect()
}

// dynamic dispatch: iterates over any type with a customer ID, collecting all
// values together
fn unique_ids_iter<I>(iterator: I) -> HashSet<u64>
where
    I: Iterator<Item = Box<dyn CustomerId>>,
{
    iterator.map(|r| r.as_ref().get_customer_id()).collect()
}
