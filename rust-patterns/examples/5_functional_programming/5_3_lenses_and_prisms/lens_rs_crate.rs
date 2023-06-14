use std::collections::HashSet;

use lens_rs::{optics, Lens, LensRef, Optics};

#[derive(Clone, Debug)]
pub struct Account {
    account_id: u32,
    account_type: String,
    // other fields omitted
}

#[derive(Clone, Debug, Lens /* derive to allow lenses to work */)]
pub struct CreditRecord {
    #[optic(ref)] // macro attribute to allow viewing this field
    customer_id: u64,
    name: String,
    dob: String,
    // other fields omitted
}

#[derive(Clone, Debug, Lens)]
pub struct AccountRecord {
    #[optic(ref)]
    customer_id: u64,
    accounts: Vec<Account>,
}

fn unique_ids_lens<T>(iter: impl Iterator<Item = T>) -> HashSet<u64>
where
    T: LensRef<Optics![customer_id], u64>, // any type with this field
{
    iter.map(|r| *r.view_ref(optics!(customer_id))).collect()
}
