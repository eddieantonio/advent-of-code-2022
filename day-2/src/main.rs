use inpt::{inpt, Inpt};

#[derive(Inpt, Debug, Clone, Copy)]
enum Move {
    #[inpt(regex = "A|X")]
    Rock,
    #[inpt(regex = "B|Y")]
    Paper,
    #[inpt(regex = "C|Z")]
    Scissors,
}

#[derive(Copy, Clone, Debug)]
enum GameResult {
    OpponentWins,
    PlayerWins,
    Draw,
}

impl Move {
    fn score(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Inpt, Debug, Copy, Clone)]
struct Round {
    opponent: Move,
    player: Move,
}

impl Round {
    fn play(self) -> (i32, i32) {
        use GameResult::*;
        use Move::*;

        let Round { opponent, player } = self;
        let result = match (opponent, player) {
            (Paper, Paper) => Draw,
            (Paper, Rock) => OpponentWins,
            (Paper, Scissors) => PlayerWins,
            (Rock, Paper) => PlayerWins,
            (Rock, Rock) => Draw,
            (Rock, Scissors) => OpponentWins,
            (Scissors, Paper) => OpponentWins,
            (Scissors, Rock) => PlayerWins,
            (Scissors, Scissors) => Draw,
        };

        match result {
            PlayerWins => (opponent.score(), player.score() + 6),
            OpponentWins => (opponent.score() + 6, player.score()),
            Draw => (3 + opponent.score(), 3 + player.score()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rounds = inpt::<Vec<Round>>("A Y B X C Z")?;

    let (_, player) = rounds
        .into_iter()
        .map(|round| round.play())
        .reduce(|(o1, p1), (o2, p2)| (o1 + o2, p1 + p2))
        .unwrap();

    println!("{:?}", player);
    Ok(())
}
