use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_top_three(top_three: &mut Vec<u32>, new_sum: u32)
{
    if  top_three.len() < 3
    {
        top_three.push(new_sum);
    }
    else if top_three[0] < new_sum
    {
        top_three[0] = new_sum;
    }
    top_three.sort(); // Wasteful sort - is 3 elements, who cares :P
}

pub fn part_2()
{
    // read file
    //
    // for each line
    //    if empty line:
    //       if top_3 is smaller than 3 elements
    //          add the sum
    //       else if the sum is larger than the smallest top_3
    //          exchange sum with hte smallest top_3
    //       sort top_3 
    //    else:
    //       add to the line to sum


    if let Ok(lines) = read_lines("input_aoc_1")
    {
        let mut top_three = vec![]; 
        let mut sum = 0;
        for line in lines
        {
            if let Ok(line_str) = line
            {
                if line_str.trim().is_empty()
                {
                    update_top_three(&mut top_three, sum);
                    sum = 0;
                }
                else
                {
                    sum += line_str.parse::<u32>().unwrap();
                }
            }
        }
        update_top_three(&mut top_three, sum); // one more time to account for the last block of calories 
        println!("ANSWER TO PART 2: {}", top_three.iter().sum::<u32>());
    }
}

pub fn part_1()
{
    // read file
    //
    // for each line
    //    if empty line:
    //       compare to max
    //       if greater than max
    //          max = sum
    //       sum = 0
    //    else
    //      add to the line to sum

    if let Ok(lines) = read_lines("input_aoc_1")
    {
        let mut sum = 0;
        let mut max = 0;
        for line in lines
        {
            if let Ok(line_str) = line
            {
                if line_str.trim().is_empty()
                {
                    if sum > max
                    {
                        max = sum;
                    }
                    sum = 0;
                }
                else
                {
                    sum += line_str.parse::<u32>().unwrap();
                }
            }
        }
        println!("ANSWER TO PART 1: {}", std::cmp::max(sum, max));
    }
}
