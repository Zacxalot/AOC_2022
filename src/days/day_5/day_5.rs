use std::fs;
use std::time::Instant;


use ndarray::{Array2};

use crate::Answer;

#[derive(Debug)]
struct Line{
    x1:i32,
    y1:i32,
    x2:i32,
    y2:i32,
}

pub fn day_5_main() -> Answer{
    let time_before = Instant::now();


    let mut lines:Vec<Line> = vec![];
    
    let mut max_x = 0;
    let mut max_y = 0;

    for line in fs::read_to_string("src/days/day_5/input1.txt").unwrap().split("\n"){
        // Split the line into a vec of 4 ints
        let vals = line.split(|c| c == ' ' || c == '-' || c == '>' || c == ',').map(|val|  val.trim()).filter(|c| c.len() > 0).map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // Convert vec to Line struct
        let new_line = Line{
            x1:*vals.get(0).unwrap(),
            y1:*vals.get(1).unwrap(),
            x2:*vals.get(2).unwrap(),
            y2:*vals.get(3).unwrap()
        }; 

        // Find the bounds of the vent map
        if new_line.x1 > max_x {max_x = new_line.x1};
        if new_line.x2 > max_x {max_x = new_line.x2};
        if new_line.y1 > max_y {max_y = new_line.y1};
        if new_line.y2 > max_y {max_y = new_line.y2};

        lines.push(new_line);
    }

    
    // Get two empty vent maps
    let mut vent_map_1 = Array2::<i32>::zeros(((max_x + 1) as usize, (max_y + 1) as usize));
    let mut vent_map_2 = Array2::<i32>::zeros(((max_x + 1) as usize, (max_y + 1) as usize));
    // let mut vent_map_2 = Array2::zeros((max_x as usize, max_y as usize));

    // For each line, find the points and add them to the map
    for line in lines.iter(){
        for point in calculate_line(&line){
            // Part 2
            vent_map_2[[point.0,point.1]] += 1;

            // Part 1, only add points which aren't diagonal
            if line.x1 == line.x2 || line.y1 == line.y2{
                vent_map_1[[point.0,point.1]] += 1;
            }
        }
    }
    

    let part_1 = vent_map_1.iter().filter(|val| val >= &&2).count();
    let part_2 = vent_map_2.iter().filter(|val| val >= &&2).count();


    let duration = Instant::now() - time_before;

    Answer{day:5, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

// Finds all of the points on the line
fn calculate_line(line:&Line) -> Vec<(usize,usize)>{
    let mut out_line:Vec<(usize,usize)> = vec![];

    // Get the gradient of each axis
    let x_gradient = (line.x2 - line.x1)/((line.x2 - line.x1).abs().max(1));
    let y_gradient = (line.y2 - line.y1)/((line.y2 - line.y1).abs().max(1));


    let mut x = line.x1;
    let mut y = line.y1;

    // Loop until we get to the other end and add a point as we go
    while x != line.x2 || y != line.y2{
        out_line.push((x as usize,y as usize));
        x += x_gradient;
        y += y_gradient;
    }
    out_line.push((x as usize,y as usize));

    out_line
}

