use adventofcode_rs::read_lines;
use eyre::Result;

fn main() -> Result<()> {
    println!("Day 01");
    println!("Reading input file");
    let lines = read_lines("input/day01.txt")?;
    let mut most_calories = 0;
    let mut last_processed_elve_cumulative_calories = 0;

    // We don't need to store accounting of each elve's calories, just the last one
    // and the most calories we've seen so far
    // That's because the solution does not involve identifying the elve with the most calories
    // Iterate over the lines
    for line in lines {
        let line = line?;
        // If the line is empty, we reached the end of an elve
        if line.is_empty() {
            // If the last elve had more calories than the most calories we've seen so far
            if last_processed_elve_cumulative_calories > most_calories {
                // Update the most calories we've seen so far
                most_calories = last_processed_elve_cumulative_calories;
            }
            // Reset the last processed elve's cumulative calories
            last_processed_elve_cumulative_calories = 0;
        }
        // If the line is not empty, we are still processing the same elve
        else {
            let calories = line.parse::<i32>()?;
            last_processed_elve_cumulative_calories += calories;
        }
    }
    println!("Most calories: {}", most_calories);
    Ok(())
}
