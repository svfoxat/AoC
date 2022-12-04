mod test;

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win(Shape),
    Lose(Shape),
    Draw(Shape),
}

enum DesiredResult {
    Win,
    Lose,
    Draw,
}

pub fn part_1(lines: &Vec<&str>) -> usize { 
    lines.iter()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            (split[0], split[1])
        })
        .map(|(opp, me)| (get_shape(opp), get_shape(me)))
        .map(|(opp, me)| play_round(opp, me))
        .sum()
}

pub fn part_2(lines: &Vec<&str>) -> usize { 
    lines.iter()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            (split[0], split[1])
        })
        .map(|(opp, res)| (get_shape(opp), get_outcome(res)))
        .map(|(opp, res)| (opp.clone(), get_shape_for_outcome(res, opp)))
        .map(|(opp, me)| play_round(opp, me))
        .sum()
}


fn play_round(opp: Shape, me: Shape) -> usize {
    match get_winner(opp, me) {
        Result::Win(v) => get_value(v) + 6,
        Result::Lose(v) => get_value(v) + 0,
        Result::Draw(v) => get_value(v) + 3,
    }
}

fn get_shape(s: &str) -> Shape {
    match s {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("wut"),
    }
}

fn get_outcome(s: &str) -> DesiredResult {
    match s {
        "X" => DesiredResult::Lose,
        "Y" => DesiredResult::Draw,
        "Z" => DesiredResult::Win,
        _ => panic!("wut"),
    }
}

fn get_shape_for_outcome(outcome: DesiredResult, opp: Shape) -> Shape {
    match outcome {
        DesiredResult::Win => match opp {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        DesiredResult::Lose => match opp {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        DesiredResult::Draw => opp,
    }
}

// Result enum with my Shape
fn get_winner(opp: Shape, me: Shape) -> Result {
    match (opp, me) {
        (Shape::Rock, Shape::Rock) => Result::Draw(Shape::Rock),
        (Shape::Rock, Shape::Paper) => Result::Win(Shape::Paper),
        (Shape::Rock, Shape::Scissors) => Result::Lose(Shape::Scissors),
        (Shape::Paper, Shape::Rock) => Result::Lose(Shape::Rock),
        (Shape::Paper, Shape::Paper) => Result::Draw(Shape::Paper),
        (Shape::Paper, Shape::Scissors) => Result::Win(Shape::Scissors),
        (Shape::Scissors, Shape::Rock) => Result::Win(Shape::Rock),
        (Shape::Scissors, Shape::Paper) => Result::Lose(Shape::Paper),
        (Shape::Scissors, Shape::Scissors) => Result::Draw(Shape::Scissors),
    }
}

fn get_value(s: Shape) -> usize {
    match s {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}