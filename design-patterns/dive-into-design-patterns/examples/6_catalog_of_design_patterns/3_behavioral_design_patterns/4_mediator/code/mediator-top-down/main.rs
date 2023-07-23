mod train_station;
mod trains;

use train_station::TrainStation;
use trains::{FreightTrain, PassengerTrain};

/// cargo r --example mediator-top-down
fn main() {
    let train1 = PassengerTrain::new("Train 1");
    let train2 = FreightTrain::new("Train 2");

    // Station has `accept` and `depart` methods,
    // but it also implements `Mediator`.
    let mut station = TrainStation::default();

    // Station is taking ownership of the trains.
    station.accept(train1);
    station.accept(train2);

    // `train1` and `train2` have been moved inside,
    // but we can use train names to depart them.
    station.depart("Train 1");
    station.depart("Train 2");
    station.depart("Train 3");

    // Passenger train Train 1: Arrived
    // Freight train Train 2: Arrival blocked, waiting
    // Passenger train Train 1: Leaving
    // Freight train Train 2: Arrived
    // Freight train Train 2: Leaving
    // 'Train 3' is not on the station!
}
