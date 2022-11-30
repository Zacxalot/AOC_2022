use std::{fs, vec};
use std::time::Instant;

use ndarray::Array2;

use crate::Answer;

pub fn day_9_main() -> Answer{
    let time_before = Instant::now();

    // Turn lines into a vec of strings
    let contents = fs::read_to_string("src/days/day_9/input1.txt").unwrap().split("\n").map(|line| line.trim().to_owned()).collect::<Vec<String>>();

    // Get the width and height of the map
    let width = contents[0].len();
    let height = contents.len();

    // Make our height and basin maps
    let mut map:Array2<u8> = Array2::zeros((width,height));
    let mut basin_map:Array2<u32> = Array2::zeros((width,height));

    // Take the values from contents and put them into map
    for (y,line) in contents.iter().enumerate(){
        for (x,val) in line.chars().enumerate(){
            map[[x,y]] = val.to_digit(10).unwrap() as u8;
        }
    }

    let mut lowpoints:Vec<(u32,(usize,usize))> = vec![];

    // Counter is used to keep track of how many seperate basins we are up to
    let mut counter = 1;
    for y in 0..height{
        for x in 0..width{
            let current_val = map[[x,y]];
            let mut low = true;
            // Checks each direction surrounding the point and checks that it's lower than all of them
            if x > 0 && map[[x-1,y]] <= current_val{
                low = false;
            }
            if low && x < width - 1 && map[[x+1,y]] <= current_val{
                low = false;
            }
            if low && y > 0 && map[[x,y-1]] <= current_val{
                low = false;
            }
            if low && y < height - 1 && map[[x,y+1]] <= current_val{
                low = false;
            }

            // Adds to the lowpoints if it is lower than surrounding
            if low{
                lowpoints.push((current_val as u32,(x,y)));
            }

            //Part 2 floodfills sections divided by "9" values
            if current_val != 9 && basin_map[[x,y]] == 0 {
                basin_map[[x,y]] = counter;
                let mut neighbors = find_surrounding(&map, width, height, x, y);
                let mut checked:Vec<(usize,usize)> = vec![(x,y)];
                let mut next_neighbors:Vec<(u32,(usize,usize))>;

                // Repeatedly sets the basin_map value of each neighbor to counter util no non "9" neighbors are unchecked
                while neighbors.len() > 0 {
                    next_neighbors = vec![];

                    for neighbor in neighbors{
                        if neighbor.0 != 9 && !checked.contains(&neighbor.1){
                            basin_map[[neighbor.1.0,neighbor.1.1]] = counter;
                            checked.push(neighbor.1);
                            next_neighbors.append(&mut find_surrounding(&map,width,height,neighbor.1.0,neighbor.1.1));
                        }
                    }

                    neighbors = next_neighbors;
                }

                counter += 1;
            }
        }
    }
    
    // Gets the frequency of each number in the basin_map
    let mut basin_counts = vec![0;lowpoints.len()];
    for val in basin_map.iter().filter(|val| val != &&0){
        basin_counts[*val as usize -1] += 1;
    }

    // Sorts counts so we can get the top 3
    basin_counts.sort_by(|a,b| b.partial_cmp(a).unwrap());

    let part_1:u32 = lowpoints.iter().map(|(val,_)| val + 1).sum();
    let part_2 = basin_counts[..3].iter().product::<u32>();

    let duration = Instant::now() - time_before;

    Answer{day:9, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

// Gets the neighbors of a specified point
fn find_surrounding(array:&Array2<u8>,width:usize,height:usize,x:usize,y:usize) -> Vec<(u32,(usize,usize))>{
    let mut surrounding:Vec<(u32,(usize,usize))> = vec![];

    if x > 0 {
        surrounding.push((array[[x-1,y]] as u32,(x-1,y)));
    }

    if x < width - 1{
        surrounding.push((array[[x+1,y]] as u32,(x+1,y)));
    }

    if y > 0{
        surrounding.push((array[[x,y-1]] as u32,(x,y-1)));
    }

    if y < height - 1{
        surrounding.push((array[[x,y+1]] as u32,(x,y+1)));
    }

    surrounding
}