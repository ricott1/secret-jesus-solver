#[derive(Debug, Clone, Copy)]
pub enum Event {
    FishCatch,
    Transfiguration,
    ResurrectionOfLazarus,
    HealingOfTheBlind,
    WalkingOnWater,
    SermonOnTheMount,
    WeddingAtCana,
    PalmSunday,
    CleansingOfTheTemple,
    LastSupper(usize), // dynamic players count for difficulty
}

impl Event {
    pub fn participation(&self) -> usize {
        match self {
            Event::FishCatch => 2,
            Event::Transfiguration => 3,
            Event::ResurrectionOfLazarus => 3,
            Event::HealingOfTheBlind => 3,
            Event::WalkingOnWater => 4,
            Event::SermonOnTheMount => 4,
            Event::WeddingAtCana => 5,
            Event::PalmSunday => 5,
            Event::CleansingOfTheTemple => 6,
            Event::LastSupper(n) => *n,
        }
    }

    pub fn difficulty(&self) -> usize {
        match self {
            Event::FishCatch => 0,
            Event::Transfiguration => 0,
            Event::ResurrectionOfLazarus => 1,
            Event::HealingOfTheBlind => 1,
            Event::WalkingOnWater => 1,
            Event::SermonOnTheMount => 2,
            Event::WeddingAtCana => 2,
            Event::PalmSunday => 3,
            Event::CleansingOfTheTemple => 3,
            Event::LastSupper(n) => n / 2,
        }
    }

    pub fn min_players(&self) -> usize {
        match self {
            Event::FishCatch => 4,
            Event::Transfiguration => 4,
            Event::ResurrectionOfLazarus => 4,
            Event::HealingOfTheBlind => 4,
            Event::WalkingOnWater => 5,
            Event::SermonOnTheMount => 6,
            Event::WeddingAtCana => 7,
            Event::PalmSunday => 8,
            Event::CleansingOfTheTemple => 9,
            Event::LastSupper(_) => 4,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Event::FishCatch => "Fish Catch",
            Event::Transfiguration => "Transfiguration",
            Event::ResurrectionOfLazarus => "Resurrection of Lazarus",
            Event::HealingOfTheBlind => "Healing of the Blind",
            Event::WalkingOnWater => "Walking on Water",
            Event::SermonOnTheMount => "Sermon on the Mount",
            Event::WeddingAtCana => "Wedding at Cana",
            Event::PalmSunday => "Palm Sunday",
            Event::CleansingOfTheTemple => "Cleansing of the Temple",
            Event::LastSupper(_) => "The Last Supper",
        }
    }
}
