use crate::team::Team;
use std::fmt::Write;

#[derive(Debug)]
pub struct Case {
    the_court: Team,
    the_prosecution: Team,
    the_defense: Team,
    courthouse: Courthouse,
    weather: Weather,
}

#[derive(Debug)]
struct Courthouse {}

#[derive(Debug)]
struct Weather {}

impl Case {
    pub fn new() -> Case {
        Case {
            the_court: Team::new_court(),
            the_prosecution: Team::new_lawyer_team("The Prosecution"),
            the_defense: Team::new_lawyer_team("The Defense"),
            courthouse: Courthouse {},
            weather: Weather {},
        }
    }

    pub fn get_case_name(&self) -> String {
        format!(
            "{} vs. {} ({} presiding)",
            self.the_prosecution.name, self.the_defense.name, self.the_court.name
        )
    }
}
