pub struct Game<'a> {
    table: &'a mut Table<'a>,
    pub players: [Player; 2],
    pub turn: usize
}

pub struct Table<'b> {
    positions: &'b Vec<Vec<Option<bool>>>
}

impl<'a> Game<'_> {

    pub fn start(table: &'a mut Table<'a>) -> Game<'a> {
        return Game {
            table,
            players: [Player::new(true), Player::new(false)],
            turn: 0
        }
    }

    pub fn play(&mut self, side: bool, position: (usize, usize)) -> bool {
        println!("Jogador {} tentando jogar ({}, {})", side, position.0, position.1);
        return self.table.fill(side, position.0, position.1);
    }

    pub fn is_over(&self) -> bool {
        for row in self.table.positions.into_iter() {

        }
        return true;
    }

}

impl<'a> Table<'_> {

    pub fn start_empty() -> Table<'static> {
        return Table {
            positions: &vec!(vec!(None::<bool>; 3); 3)
        }
    }

    pub fn fill(&mut self, side: bool, row: usize, col: usize) -> bool {
        if self.positions[row][col] == None {
            self.positions[row][col] = Some(side);
            return true;
        }
        return false;
    }
}

pub struct Player {
    pub side: bool
}

impl Player {

    pub fn new(side: bool) -> Player {
        return Player {
            side
        };
    }

    pub fn choose_position(&self) -> (usize, usize) {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        return (rng.gen::<usize>() % 3, rng.gen::<usize>() % 3);
    }
}