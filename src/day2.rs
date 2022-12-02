use std::fs::File;
use std::io::Read;

struct Scores {
    rock: u32,
    paper: u32,
    scissors: u32,
    lost: u32,
    draw: u32,
    won: u32   
}

pub fn solve() {
    let path = "./day2/input.txt";
    let mut file: File = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut my_score = 0u32;
    let scores = Scores {
        rock: 1,
        paper: 2,
        scissors: 3,
        lost: 0,
        draw: 3,
        won: 6
    };

    let rounds: Vec<&str> = contents.split("\n").collect();
    for round in rounds {
        let r: Vec<&str> = round.split(" ").collect();
        let op = r[0].trim();
        let my = r[1].trim();

        match op {
            "A" => match my {
                "X" => my_score += scores.rock + scores.draw,
                "Y" => my_score += scores.paper + scores.won,
                "Z" => my_score += scores.scissors + scores.lost,
                _ => panic!("nani?")
            },
            "B" => match my {
                "X" => my_score += scores.rock + scores.lost,
                "Y" => my_score += scores.paper + scores.draw,
                "Z" => my_score += scores.scissors + scores.won,
                _ => panic!("nani?")
            },
            "C" => match my {
                "X" => my_score += scores.rock + scores.won,
                "Y" => my_score += scores.paper + scores.lost,
                "Z" => my_score += scores.scissors + scores.draw,
                _ => panic!("nani?")
            },

            _ => panic!("nani?"),
        }
    }

    println!("{}", my_score);
}