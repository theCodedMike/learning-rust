use crate::department::{Cashier, Department, Doctor, Medical, Reception};
use crate::patient::Patient;

mod department;
mod patient;

/// cargo r --example cor
fn main() {
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    // Reception handles a patient passing him to the next link in the chain.
    // Reception -> Doctor -> Medical -> Cashier.
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);

    // Reception registering a patient John
    // Doctor checking a patient John
    // Medical giving medicine to a patient John
    // Cashier getting money from a patient John
    //
    // The patient has been already handled:
    //
    // Patient registration is already done
    // A doctor checkup is already done
    // Medicine is already given to a patient
    // Payment done
}
