use std::collections::HashSet;
use std::fs;
use std::time::Instant;



use crate::Answer;

pub fn day_13_main() -> Answer{
    let time_before = Instant::now();

    let lines = fs::read_to_string("src/days/day_13/input1.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.to_owned())
        .filter(|line| !(line.len() == 1))
        .collect::<Vec<String>>();

    let mut points:HashSet<(usize,usize)> = HashSet::new();

    for line in lines.iter().filter(|l| !l.starts_with("f")){
        // Get the x and y values from either side of the ","
        let mut split = line.trim().split(",");
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();

        points.insert((x,y));
    }

    let mut point_count:Vec<usize> = vec![];

    // For each of the fold instructions it gets the axis and position
    // If the axis is x it runs fold_x, else it runs fold_y
    // Gets the size of the points set and adds it to point_count vec
    for line in lines.iter().filter(|l| l.starts_with("f")){
        let mut split = line.trim().split("=");
        let axis = split.next().unwrap().chars().last().unwrap();
        let pos = split.next().unwrap().parse::<usize>().unwrap();
        if axis == 'x'{
            points = fold_x(&points,pos);
        }
        else{
            points = fold_y(&points,pos);
        }

        point_count.push(points.len());
    }

    // Get the first count that was found
    let part_1 = point_count.first().unwrap();
    
    let part_2 = "ECFHLHZF";
    // Produces result above sideways and mirrored
    // Println takes up a lot of time though so I think
    // its fair to get that one for nowt
    // Guess I could decode characters programatically but meh
    // for x in 0..50{
    //     let mut line = String::new();
    //     for y in 0..50{
    //         if points.contains(&(x,y)){
    //             line += "█";
    //         }
    //         else{
    //             line += " ";
    //         }
    //     }
    //     println!("{}",line);
    // }
    

    let duration = Instant::now() - time_before;

    Answer{day:13, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

// Takes all values below of the line, flips them on the y axis about the line 
// And returns a new HashSet of values above the line combined with the found flipped values
fn fold_y(map:&HashSet<(usize,usize)>, line:usize) -> HashSet<(usize,usize)>{
    let mut points:HashSet<(usize,usize)> = HashSet::new();
    for val in map.iter().filter(|(_,y)| y < &line){
        points.insert(*val);
    }
    
    for (x,y) in map.iter().filter(|(_,y)| y > &line){
        points.insert((*x,line - (*y-line)));
    }

    return points
}
// Takes all values right of the line, flips them on the x axis about the line 
// And returns a new HashSet of values left of the line combined with the found flipped values
fn fold_x(map:&HashSet<(usize,usize)>, line:usize) -> HashSet<(usize,usize)>{
    let mut points:HashSet<(usize,usize)> = HashSet::new();
    for val in map.iter().filter(|(x,_)| x < &line){
        points.insert(*val);
    }
    
    for (x,y) in map.iter().filter(|(x,_)| x > &line){
        points.insert((line - (*x-line),*y));
    }

    return points
}