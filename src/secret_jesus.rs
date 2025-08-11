use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;
use strum_macros::{Display, EnumIter};

use crate::Event;

pub const EVENTS_PER_GAME: usize = 5;

#[derive(Debug, Clone, Copy, Display, PartialEq, EnumIter)]
pub enum Role {
    Jesus,
    Judas,
    Peter,
    Apostle,
    Mary,
}

impl Role {
    pub const NUMBER: usize = 5;
    pub fn initial_prodigies(&self) -> [Prodigy; EVENTS_PER_GAME] {
        match self {
            Self::Jesus => [const { Prodigy::Boon }; 5],
            Self::Judas => [
                Prodigy::Bane,
                Prodigy::Bane,
                Prodigy::Bane,
                Prodigy::Null,
                Prodigy::Boon,
            ],
            Self::Peter => [
                Prodigy::Bane,
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Boon,
            ],
            Self::Apostle => [
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Boon,
            ],
            Self::Mary => [
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Null,
                Prodigy::Boon,
                Prodigy::Boon,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Prodigy {
    Bane,
    Null,
    Boon,
}

impl Prodigy {
    pub const NUMBER: usize = 3;
    pub fn value(&self) -> i32 {
        match self {
            Prodigy::Bane => -1,
            Prodigy::Null => 0,
            Prodigy::Boon => 1,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub role: Role,
    pub prodigies: Vec<Prodigy>,
}

impl Player {
    fn new(id: usize, role: Role) -> Self {
        let prodigies = role.initial_prodigies().to_vec();

        Player {
            id,
            role,
            prodigies,
        }
    }

    pub fn name(&self) -> String {
        match &self.role {
            Role::Apostle => format!("Apostle {}", self.id),
            Role::Jesus => "Jesus".into(),
            Role::Judas => "Judas".into(),
            Role::Peter => "Peter".into(),
            Role::Mary => "Mary".into(),
        }
    }

    pub fn consume_prodigy<R: rand::Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        event_index: usize,
    ) -> Prodigy {
        if event_index == EVENTS_PER_GAME - 1 {
            if self.role == Role::Judas {
                if let Some(pos) = self.prodigies.iter().position(|p| *p == Prodigy::Bane) {
                    return self.prodigies.remove(pos);
                }
            }

            if let Some(pos) = self.prodigies.iter().position(|p| *p == Prodigy::Boon) {
                return self.prodigies.remove(pos);
            }
        }

        let index = rng.random_range(0..self.prodigies.len());
        self.prodigies.remove(index)
    }
}

pub fn get_roles(n: usize) -> Vec<Role> {
    if n < 4 || n > 10 {
        panic!("Number of players must be between 4 and 10");
    }
    match n {
        4..=6 => {
            let mut roles = vec![Role::Jesus, Role::Judas, Role::Peter];
            roles.extend(vec![Role::Apostle; n - 3]);
            roles
        }
        _ => {
            let mut roles = vec![Role::Jesus, Role::Judas, Role::Peter, Role::Mary];
            roles.extend(vec![Role::Apostle; n - 4]);
            roles
        }
    }
}

pub fn get_players(n: usize) -> Vec<Player> {
    get_roles(n)
        .into_iter()
        .enumerate()
        .map(|(id, role)| Player::new(id, role))
        .collect()
}

pub fn get_events<R: rand::Rng + ?Sized>(rng: &mut R, n: usize) -> Vec<Event> {
    let all_events = vec![
        Event::FishCatch,
        Event::Transfiguration,
        Event::ResurrectionOfLazarus,
        Event::HealingOfTheBlind,
        Event::WalkingOnWater,
        Event::SermonOnTheMount,
        Event::WeddingAtCana,
        Event::PalmSunday,
        Event::CleansingOfTheTemple,
    ];

    let eligible_events: Vec<Event> = all_events
        .iter()
        .filter(|e| e.min_players() <= n)
        .map(|e| *e)
        .collect();

    let mut selected_events: Vec<Event> = eligible_events
        .choose_multiple(rng, EVENTS_PER_GAME - 1)
        .map(|e| *e)
        .collect();

    selected_events.shuffle(rng);
    selected_events.push(Event::LastSupper(n));
    selected_events
}
