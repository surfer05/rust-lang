use std::collections::HashMap;
use std::io;

// fn median_and_mode(v: &mut Vec<i32>) -> Vec<&mut i32> {
//     v.sort();
//     let mid_index: i32 = v.len().try_into().unwrap();
//     let median: i32 = v[mid_index];

//     let mut map = HashMap::new();

//     for element in v {
//         let count = map.entry(element).or_insert(0);
//         *count += 1;
//     }

//     let mut mode = 0;
//     for (k,v) in map {

//     }
// }

fn main() {
    println!("Type a sentence you want in Pig Latin");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!!!");

    let result = pig_latin(&input);

    println!("{result}");

    let v: Vec<i32> = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
    let ans = hehe(&v);
    for element in ans {
        println!("{}", element);
    }
}

fn pig_latin(s: &str) -> String {
    let mut letters = s.chars();
    let first_letter = letters.next().unwrap();

    let first_letter_lowercase = first_letter.to_lowercase().next().unwrap();

    match first_letter_lowercase {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", s),
        _ => format!("{}-{}ay", letters.as_str(), first_letter_lowercase),
    }
}

/// Mean Median and Mode
fn hehe(v: &Vec<i32>) -> Vec<f32> {
    let avg: f32;
    let median: f32;
    let mut mode: f32 = 0.0;

    {
        // calculate average
        let mut sum: i32 = 0;
        for n in v {
            sum += n;
        }
        avg = sum as f32 / v.len() as f32;
    }
    {
        // calculating median
        let mid = v.len() / 2;
        median = v[mid] as f32;
    }

    {
        // calculate mode
        let mut times = HashMap::new();

        // count
        for x in v {
            let count = times.entry(*x as usize).or_insert(0);
            *count += 1;
        }

        let mut best: i32 = 0;

        for x in times.iter() {
            if *x.1 > best {
                best = *x.1;
                mode = (*x.0) as f32;
            }
        }
    }
    let ans = vec![avg, median, mode];
    ans
}
