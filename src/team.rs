use crate::modifiers::Modifier;
use crate::player::Player;

#[derive(Debug)]
pub(crate) struct Team {
    pub(crate) name: String,
    positions: Vec<Position>,
    modifiers: Vec<Modifier>,
}

#[derive(Debug)]
struct Position {
    name: String,
    max_players: Option<u8>,
    visible: bool,
    players: Vec<Player>,
}

impl Position {
    fn new(name: &str, max_players: Option<u8>, visible: bool) -> Position {
        Position {
            name: name.to_string(),
            max_players,
            visible,
            players: vec![],
        }
    }

    fn add_player(&mut self, player: Player) {
        match self.max_players {
            None => {
                self.players.push(player);
            }
            Some(max_pl) => {
                if self.players.len() < max_pl as usize {
                    self.players.push(player);
                } else {
                    eprintln!("Position already filled!")
                }
            }
        }
    }
}

impl Team {
    pub fn new_court() -> Team {
        let mut judge = Position::new("Judge", Some(1), true);
        judge.add_player(Player::new());

        let mut executioner = Position::new("Executioner", Some(1), true);
        executioner.add_player(Player::new());

        let mut stenographer = Position::new("Stenographer", Some(1), false);
        stenographer.add_player(Player::new());

        let mut jury = Position::new("Jury", None, true);
        // add a random number of jury members (4 to 7 for now)
        let num_jury = (rand::random::<f32>() * 3.0 + 4.0) as u8;
        for _ in 0..num_jury {
            jury.add_player(Player::new());
        }

        Team {
            name: String::from("The Court"),
            positions: vec![judge, executioner, jury, stenographer],
            modifiers: vec![],
        }
    }

    pub fn new_lawyer_team(name: &str) -> Team {
        let mut lawyers = Position::new("Lawyers", Some(1), true);
        // add a random number of lawyers (2 to 5 for now)
        let num_lawyers = (rand::random::<f32>() * 3.0 + 2.0) as u8;
        for _ in 0..num_lawyers {
            lawyers.add_player(Player::new());
        }

        let mut witnesses = Position::new("Witnesses", Some(1), true);
        // add a random number of witnesses (4 to 6 for now)
        let num_witnesses = (rand::random::<f32>() * 2.0 + 4.0) as u8;
        for _ in 0..num_witnesses {
            witnesses.add_player(Player::new());
        }

        let mut shadows = Position::new("Shadows", Some(1), false);
        // add a random number of shadows members (2 to 5 for now)
        let num_shadows = (rand::random::<f32>() * 3.0 + 2.0) as u8;
        for _ in 0..num_shadows {
            shadows.add_player(Player::new());
        }

        Team {
            name: String::from(name),
            positions: vec![lawyers, witnesses, shadows],
            modifiers: vec![],
        }
    }
}
