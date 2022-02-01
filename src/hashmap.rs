use std::collections::HashMap;

pub fn learning_hashmap() {
    println!("Start to learning hashmap");

    // create hash map
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 10);

    // use collection method to create hashmap
    let teams: Vec<String> = Vec::from([String::from("Blue"), String::from("Yellow")]);
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}