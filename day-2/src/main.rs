use inpt::Inpt;

#[derive(Inpt, Debug, Clone, Copy)]
enum Move {
    #[inpt(regex = "A")]
    Rock,
    #[inpt(regex = "B")]
    Paper,
    #[inpt(regex = "C")]
    Scissors,
}

#[derive(Inpt, Debug, Clone, Copy)]
enum PlayerMust {
    #[inpt(regex = "X")]
    Lose,
    #[inpt(regex = "Y")]
    Draw,
    #[inpt(regex = "Z")]
    Win,
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
    requirement: PlayerMust,
}

impl Round {
    fn play(self) -> (i32, i32) {
        use GameResult::*;
        use Move::*;

        let Round {
            opponent,
            requirement,
        } = self;

        let player = match (opponent, requirement) {
            (m, PlayerMust::Draw) => m,
            (Rock, PlayerMust::Lose) => Scissors,
            (Rock, PlayerMust::Win) => Paper,
            (Paper, PlayerMust::Lose) => Rock,
            (Paper, PlayerMust::Win) => Scissors,
            (Scissors, PlayerMust::Lose) => Paper,
            (Scissors, PlayerMust::Win) => Rock,
        };

        match shoot(opponent, player) {
            PlayerWins => (opponent.score(), player.score() + 6),
            OpponentWins => (opponent.score() + 6, player.score()),
            Draw => (3 + opponent.score(), 3 + player.score()),
        }
    }
}

fn shoot(opponent: Move, player: Move) -> GameResult {
    use GameResult::*;
    use Move::*;

    match (opponent, player) {
        (Paper, Paper) => Draw,
        (Paper, Rock) => OpponentWins,
        (Paper, Scissors) => PlayerWins,
        (Rock, Paper) => PlayerWins,
        (Rock, Rock) => Draw,
        (Rock, Scissors) => OpponentWins,
        (Scissors, Paper) => OpponentWins,
        (Scissors, Rock) => PlayerWins,
        (Scissors, Scissors) => Draw,
    }
}

#[inpt::main]
fn main(rounds: Vec<Round>) -> Result<(), Box<dyn std::error::Error>> {
    let (_, player) = rounds
        .into_iter()
        .map(|round| round.play())
        .reduce(|(o1, p1), (o2, p2)| (o1 + o2, p1 + p2))
        .unwrap();

    println!("{:?}", player);
    Ok(())
}
