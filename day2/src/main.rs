use std::str::FromStr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let rounds: Vec<Round> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;
    let total_score: usize = rounds.iter().map(|r| r.our_score()).sum();
    dbg!(total_score);

    let rounds: Vec<_> = include_str!("input.txt")
        .lines()
        .map(Round::from_str)
        .collect::<Result<_, _>>()?;
    let total_score: usize = rounds.iter().map(|r| r.our_score()).sum();
    dbg!(total_score);

    let mut total_score = 0;
    for round in include_str!("input.txt").lines().map(Round::from_str) {
        total_score += round?.our_score();
    }
    dbg!(total_score);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

// impl TryFrom<char> for Move {
//     type Error = color_eyre::Report;

//     fn try_from(c: char) -> Result<Self, Self::Error> {
//         match c {
//             'A' | 'X' => Ok(Move::Rock),
//             'B' | 'Y' => Ok(Move::Paper),
//             'C' | 'Z' => Ok(Move::Scissors),
//             _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
//         }
//     }
// }

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
                | (Self::Paper, Self::Rock)
        )
    }

    fn outcome(self, theirs: Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move")
    }

    fn drawing_move(self) -> Self {
        self
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Win => theirs.winning_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Loss => theirs.losing_move(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

// impl FromStr for Round {
//     type Err = color_eyre::Report;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut chars = s.chars();
//         let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
//             return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
//         };

//         Ok(Self {
//             theirs: theirs.try_into()?,
//             ours: ours.try_into()?,
//         })
//     }
// }

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<outcome>EOF, got {s:?}"));
        };
        let theirs = Move::try_from(theirs)?;
        let outcome = Outcome::try_from(outcome)?;
        let ours = outcome.matching_move(theirs);

        Ok(Self { theirs, ours })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}
