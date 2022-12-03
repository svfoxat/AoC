#[cfg(test)]
pub mod test {
    use std::time::Instant;
    use std::fs;

    #[test]
    fn test_1 () {
        let file = fs::read_to_string("input.txt").unwrap();
        let lines: Vec<&str> = file.lines().collect();
        
        let start_1 = Instant::now();
        let part_1 = crate::part_1(&lines);
        let end_1 = start_1.elapsed();
    
        let start_2 = Instant::now();
        let part_2 = crate::part_2(&lines);
        let end_2 = start_2.elapsed();
    
        println!("Part 1: {}, took: {}µs", part_1, end_1.as_micros());
        println!("Part 2: {}, took: {}µs", part_2, end_2.as_micros());
    }
}

