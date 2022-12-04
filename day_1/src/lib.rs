use std::vec;
mod test;

pub fn part_1(lines: &Vec<&str>) -> usize {
    let elves = cumulate_elves(lines);
    elves[0] as usize
}

pub fn part_2(lines: &Vec<&str>) -> usize {
    let elves = cumulate_elves(lines);
    let top_3 = &elves[0..3].to_vec().into_iter().sum::<i32>();

    *top_3 as usize
}

fn cumulate_elves(lines: &Vec<&str>) -> Vec<i32> {
    let it = lines.iter();
    let mut elves: Vec<i32> = vec![0];
    let mut curr_elv: usize = 0;

    for line in it {
        if line.is_empty() {
            curr_elv += 1;
            elves.push(0);
            continue;
        }

        let val = line.parse::<i32>().unwrap();
        elves[curr_elv] += val;
    }

    elves.sort_by(|a, b| b.cmp(a));
    elves
}