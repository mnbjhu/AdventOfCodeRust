use std::env;
use std::fs;

static A: u8 = 65;
static X: u8 = 88;

fn main(){
    part1();
}

fn part1() {
    let file_path = "C:\\Users\\james\\IdeaProjects\\AdventOfCodeRust\\src\\input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut score: i32 = 0;
    let mut is_mine = true;
    let mut ops: u8 = 0;
    for byte in contents.as_bytes() {
        if *byte >= A {
            is_mine = !is_mine;
            if is_mine {
                let mine = byte + 1 - X;
                let round_win = (3 + mine - ops) % 3;
                score += i32::from(round_win*3 + mine);
            } else { ops = byte - A };
        }
    }
    println!("{}",score)
}

