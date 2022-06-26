use crate::core::exercise::Exercise;
use crate::core::meal::Meal;
use crate::core::Mental::Mental;
use crate::core::work::Work;
use crate::core::sleep::Sleep;

enum Week {
    Mon,
    Tues,
    Wed,
    Thurs,
    Fri,
    Sat,
    Sun,
}

pub struct Day {
    date: Box<str>,
    day: Week,
    exercises: Box<[Exercise]>,
    meals: Box<[Meal]>,
    work: Box<[Work]>,
    mental: Box<[Mental]>,
    sleep: Sleep,
}

impl Day {

}