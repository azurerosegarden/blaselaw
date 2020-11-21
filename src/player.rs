use rand::prelude::*;
use rand_distr::{Beta, Distribution};
use std::f32;
use std::fmt;
use std::fmt::Formatter;
use std::io::repeat;

#[derive(Debug)]
struct RawStars {
    litigation: f32,
    judgement: f32,
    baserunning: f32,
    defense: f32,
}

#[derive(Debug)]
struct Stars {
    litigation: u8,
    judgement: u8,
    baserunning: u8,
    defense: u8,
}

#[derive(Debug)]
struct Stats {
    base_thirst: f32,
    continuation: f32,
    ground_friction: f32,
    indulgence: f32,
    laserlikeness: f32,
    anticapitalism: f32,
    chasiness: f32,
    omniscience: f32,
    tenaciousness: f32,
    watchfulness: f32,
    buoyancy: f32,
    divinity: f32,
    martyrdom: f32,
    moxie: f32,
    musclitude: f32,
    patheticism: f32,
    thwackability: f32,
    tragicness: f32,
    coldness: f32,
    overpowerment: f32,
    ruthlessness: f32,
    shakespearianism: f32,
    suppression: f32,
    unthwackability: f32,
    total_fingers: u8,
    cinnamon: f32,
    deceased: bool,
    fate: f32,
    peanut_allergy: bool,
    pressurization: f32,
    soul: u8,
    raw_stars: RawStars,
    pub stars: Stars,
}

impl Stats {
    pub fn new() -> Stats {
        //create random stats
        let beta: Beta<f32> = Beta::new(3.5, 2.0).unwrap();

        let mut temp = Stats {
            base_thirst: beta.sample(&mut rand::thread_rng()),
            continuation: beta.sample(&mut rand::thread_rng()),
            ground_friction: beta.sample(&mut rand::thread_rng()),
            indulgence: beta.sample(&mut rand::thread_rng()),
            laserlikeness: beta.sample(&mut rand::thread_rng()),
            anticapitalism: beta.sample(&mut rand::thread_rng()),
            chasiness: beta.sample(&mut rand::thread_rng()),
            omniscience: beta.sample(&mut rand::thread_rng()),
            tenaciousness: beta.sample(&mut rand::thread_rng()),
            watchfulness: beta.sample(&mut rand::thread_rng()),
            buoyancy: beta.sample(&mut rand::thread_rng()),
            divinity: beta.sample(&mut rand::thread_rng()),
            martyrdom: beta.sample(&mut rand::thread_rng()),
            moxie: beta.sample(&mut rand::thread_rng()),
            musclitude: beta.sample(&mut rand::thread_rng()),
            patheticism: beta.sample(&mut rand::thread_rng()),
            thwackability: beta.sample(&mut rand::thread_rng()),
            tragicness: 0.1,
            coldness: beta.sample(&mut rand::thread_rng()),
            overpowerment: beta.sample(&mut rand::thread_rng()),
            ruthlessness: beta.sample(&mut rand::thread_rng()),
            shakespearianism: beta.sample(&mut rand::thread_rng()),
            suppression: beta.sample(&mut rand::thread_rng()),
            unthwackability: beta.sample(&mut rand::thread_rng()),
            total_fingers: 0,
            cinnamon: beta.sample(&mut rand::thread_rng()),
            deceased: false,
            fate: beta.sample(&mut rand::thread_rng()),
            peanut_allergy: false,
            pressurization: beta.sample(&mut rand::thread_rng()),
            soul: 0,
            raw_stars: RawStars {
                litigation: 0.0,
                judgement: 0.0,
                baserunning: 0.0,
                defense: 0.0,
            },
            stars: Stars {
                litigation: 0,
                judgement: 0,
                baserunning: 0,
                defense: 0,
            },
        };

        // update the stars to match
        temp.derive_the_stars();

        temp
    }

    pub fn derive_the_stars(&mut self) {
        // derive the stars - uses modified algorithms based on the ones extrapolated by @baronblissy

        // litigation
        self.raw_stars.litigation = f32::powf((1.0 - self.tragicness), 0.01)
            * f32::powf((1.0 - self.patheticism), 0.05)
            * f32::powf((self.thwackability * self.divinity), 0.35)
            * f32::powf((self.moxie * self.musclitude), 0.075)
            * f32::powf(self.martyrdom, 0.02);

        // judgement
        self.raw_stars.judgement = f32::powf(self.unthwackability, 0.5)
            * f32::powf(self.ruthlessness, 0.4)
            * f32::powf(self.overpowerment, 0.15)
            * f32::powf(self.shakespearianism, 0.1)
            * f32::powf(self.coldness, 0.025);

        // baserunning
        self.raw_stars.baserunning = f32::powf(self.laserlikeness, 0.5)
            * f32::powf(
                (self.base_thirst * self.continuation * self.ground_friction * self.indulgence),
                0.1,
            );

        // defense
        self.raw_stars.defense = f32::powf((self.omniscience * self.tenaciousness), 0.2)
            * f32::powf(
                (self.watchfulness * self.anticapitalism * self.chasiness),
                0.1,
            );

        // convert to a real number of stars
        self.stars.litigation = (10.0 * self.raw_stars.litigation).round() as u8;
        self.stars.judgement = (10.0 * self.raw_stars.judgement).round() as u8;
        self.stars.baserunning = (10.0 * self.raw_stars.baserunning).round() as u8;
        self.stars.defense = (10.0 * self.raw_stars.defense).round() as u8;
    }
}

#[derive(Debug)]
pub(crate) struct Player {
    name: String,
    stats: Stats,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "Player Name".to_string(),
            stats: Stats::new(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.name, self.stats.stars)
    }
}

impl fmt::Display for Stars {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Litigation: {}\nJudgement: {}\nLitigation: {}\nDefense: {}",
            self.litigation, self.judgement, self.baserunning, self.defense
        )
    }
}
