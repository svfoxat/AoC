use std::env;
use std::fs;

fn main () {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    
    let part_1 = part_1(&lines);
    let part_2 = part_2(&lines);


    println!("Part 1: {}\t", part_1);
    println!("Part 2: {}\t", part_2);
}


fn part_2(lines: &Vec<&str>) -> u32 {
    let chunks = lines.chunks(3);
    chunks.into_iter()
        .map(|chunk| {
            let common: Vec<char> = chunk[0].chars().filter(|c| chunk[1].contains(*c) && chunk[2].contains(*c)).collect();
            common[0]
        })
        .map(|common_item: char | get_priority_for_char(&common_item))
        .sum::<u32>()
}

fn part_1(lines: &Vec<&str>) -> u32 {
   lines.into_iter().map(|l| (&l[0..l.len()/2], &l[l.len()/2..l.len()]))
    .map(|(slice_1, slice_2)| slice_1.chars().into_iter().filter(|c| slice_2.contains(*c)).next().unwrap())
    .map(|s: char| get_priority_for_char(&s))
    .sum::<u32>()
}

fn get_priority_for_char(char: &char) -> u32 {
    [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
    .iter()
    .position(|&r| r == *char).unwrap() as u32 + 1
}