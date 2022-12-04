mod test;

pub fn part_1(lines: &Vec<&str>) -> usize {
    lines.into_iter()
        .map(|line| {
            let split = line.split(",").collect::<Vec<&str>>();
            (split[0], split[1])
        })
        .map(|(split_0, split_1)| {
            let s0: Vec<u32> = split_0.split("-").map(|s| s.parse::<u32>().unwrap()).collect();
            let s1: Vec<u32> = split_1.split("-").map(|s| s.parse::<u32>().unwrap()).collect();
            (s0[0], s0[1], s1[0], s1[1])
        })
        .filter(|(s0_0, s0_1, s1_0, s1_1)| {
            check_if_contained(s0_0, s0_1, s1_0, s1_1)
        })
        .count()
}

pub fn part_2(lines: &Vec<&str>) -> usize {
    lines.into_iter()
    .map(|line| {
        let split = line.split(",").collect::<Vec<&str>>();
        (split[0], split[1])
    })
    .map(|(split_0, split_1)| {
        let s0: Vec<u32> = split_0.split("-").map(|s| s.parse::<u32>().unwrap()).collect();
        let s1: Vec<u32> = split_1.split("-").map(|s| s.parse::<u32>().unwrap()).collect();
        (s0[0], s0[1], s1[0], s1[1])
    })
    .filter(|(s0_0, s0_1, s1_0, s1_1)| {
        check_if_overlap(s0_0, s0_1, s1_0, s1_1)
    })
    .count()
}


fn check_if_contained(s0_0: &u32, s0_1: &u32, s1_0: &u32, s1_1: &u32) -> bool {
    s0_0 <= s1_0 && s0_1 >= s1_1 || s1_0 <= s0_0 && s1_1 >= s0_1
}
fn check_if_overlap(s0_0: &u32, s0_1: &u32, s1_0: &u32, s1_1: &u32) -> bool {
    s0_0 <= s1_0 && s0_1 >= s1_0 || s1_0 <= s0_0 && s1_1 >= s0_0
}
