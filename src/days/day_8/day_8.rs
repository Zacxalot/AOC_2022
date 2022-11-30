use std::fs;
use std::time::Instant;

use crate::Answer;

pub fn day_8_main() -> Answer{
    let time_before = Instant::now();

    // Gets a vec of each line
    // Each digit on each line is converted to a u8 using digit_chars_to_num
    // This value is tupled with the number of ones it has in it
    let contents = fs::read_to_string("src/days/day_8/input1.txt")
                      .unwrap()
                      .split("\n")
                      .map(|line| line.split(&[' ','|','\r'][..])
                                           .filter(|val| val != &"")
                                           .map(|val| digit_chars_to_num(val))
                                           .map(|val| (val,val.count_ones() as u8))
                                           .collect::<Vec<(u8,u8)>>())
                      .collect::<Vec<Vec<(u8,u8)>>>();

    // Iterates through the list and counts how many output values "lengths" (Now the number of ones in the number) equal 2, 3, 4 or 7
    let part_1 = contents.iter()
                         .map(|line| line[10..14].iter().filter(|(_,ones)| [2,3,4,7].contains(ones)).count())
                         .sum::<usize>();

    // Deduces the values of each line and sums their final value
    let part_2 = contents.iter().map(|digits| deduce_values(digits)).sum::<u32>();

    let duration = Instant::now() - time_before;

    Answer{day:8, part_1:part_1.to_string(), part_2:part_2.to_string(), duration:duration}
}

fn deduce_values(digits:&Vec<(u8,u8)>) -> u32{
    let mut values:[u8; 10] = [0;10];
    // Get all of the values we know because of their length
    values[1] = digits.iter().find(|(_,ones)| ones == &2).unwrap().0;
    values[4] = digits.iter().find(|(_,ones)| ones == &4).unwrap().0;
    values[7] = digits.iter().find(|(_,ones)| ones == &3).unwrap().0;
    values[8] = digits.iter().find(|(_,ones)| ones == &7).unwrap().0;

    // 3 has 5 ones and has 3 bits different from 1
    values[3] = digits.iter().find(|(val,ones)| ones == &5 && (val ^ values[1]).count_ones() == 3).unwrap().0;
    
    // 9 has 6 digits and one bit different to 3
    values[9] = digits.iter().find(|(val,ones)| ones == &6 && (val ^ values[3]).count_ones() == 1).unwrap().0;
    
    // 5 has 5 bits, 1 bit different to 9 and isn't the same value as 3
    values[5] = digits.iter().find(|(val,ones)| ones == &5 && (val ^ values[9]).count_ones() == 1 && val != &values[3]).unwrap().0;

    // 2 has 5 bits and isn't 3 or 5
    values[2] = digits.iter().find(|(val,ones)| ones == &5 && val != &values[3] && val != &values[5]).unwrap().0;

    // c is the bit shared by 1 and 2
    let c = values[2] & values[1];

    // 6 has 6 bits and none of them are the same as c
    values[6] = digits.iter().find(|(val,ones)| ones == &6 && val & c == 0).unwrap().0;

    // 0 is 6 digits and not 6 or 9
    values[0] = digits.iter().find(|(val,ones)| ones == &6 && val != &values[9] && val != &values[6]).unwrap().0;

    let mut digit_string = "".to_string();

    // Loop through the output digits and find their value from the values array
    for (digit,_) in digits[10..14].iter(){
        digit_string += &values.iter().enumerate().find(|(_, val)| val == &digit).unwrap().0.to_string();
    }

    digit_string.parse::<u32>().unwrap()
}


// Assigns each letter to a bit, makes bitwise operations possible
fn digit_chars_to_num(line:&str) -> u8{
    let mut out:u8 = 0;

    for char in line.chars(){
        out += match char{
            'a' => 0b1000000,
            'b' => 0b0100000,
            'c' => 0b0010000,
            'd' => 0b0001000,
            'e' => 0b0000100,
            'f' => 0b0000010,
            'g' => 0b0000001,
            _ => 0b1111111
        }
    }

    out
}