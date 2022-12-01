use adventofcode_rs::read_lines;
use eyre::Result;

fn main() -> Result<()> {
    println!("Day 02");
    println!("Reading input file");
    let lines = read_lines("input/day01.txt")?;

    let mut top3 = [0, 0, 0];
    let mut last_elve_count = 0;
    for line in lines {
        let line = line?;
        if line.is_empty() {
            if last_elve_count > top3[0] {
                top3[2] = top3[1];
                top3[1] = top3[0];
                top3[0] = last_elve_count;
            } else if last_elve_count > top3[1] {
                top3[2] = top3[1];
                top3[1] = last_elve_count;
            } else if last_elve_count > top3[2] {
                top3[2] = last_elve_count;
            }
            last_elve_count = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();
            last_elve_count += calories;
        }
    }

    println!("Top 3: {:?}", top3);
    let total = top3[0] + top3[1] + top3[2];
    println!("Total calories: {}", total);
    Ok(())
}
