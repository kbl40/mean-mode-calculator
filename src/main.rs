use std::collections::HashMap;
use std::io;

fn main() {
    // User inputs data and write to vector
    let mut v = Vec::new();

    // `for` loop to collect five integers from the user and insert into a vector
    for _i in 1..6 {
        println!("Please input an integer.");

        let mut integer = String::new();

        io::stdin()
            .read_line(&mut integer)
            .expect("Failed to read line");
    
        let integer: i32 = match integer.trim().parse() {
            Ok(num) => num,
            // Error checking
            Err(_) => continue,
        };

        v.push(integer);
    }
    
    let mean_fun = mean(&v);
    println!("The mean is {}", mean_fun);

    let mode_fun = mode(&v);
    println!("The mode is {}", mode_fun);
}

// Pass a `Vec` as a reference to calculate the mean of the vector
fn mean(vec: &Vec<i32>) -> f32 {
    let total: i32 = vec.iter().sum();
    let length = vec.len();
    total as f32 / length as f32
}

// Pass a `Vec` as a reference to calculate the mode. 
fn mode(vec: &Vec<i32>) -> i32 {
    let mut count_map = HashMap::new();

    for val in vec {
        let count = count_map.entry(val).or_insert(0);
        *count += 1;
    }

    //println!("{:?}", count_map);

    let mut max = 0;
    let mut key_max = 0;

    for (key, value) in &count_map {
        if value > &max {
            max = *value;
            key_max = **key;
        }
    }

    key_max
}
