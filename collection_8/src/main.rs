use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vector part
    {
        let mut vec = vec![1, 2, 3];
        vec.push(1);
    }
    // vec.push(2); // out of scope

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // will panic
    let does_not_exist = v.get(5);
    match does_not_exist {
        Some(foo) => println!("{}", foo),
        None => println!("None :'("),
    }

    let mut v = vec![100, 50, 1];

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }

    //String part
    let data = "initial contents";
    let s = data.to_string();
    let mut s = String::from("initial contents");

    s.push_str(" foo");
    println!("{}", s);

    s.push('a');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("{}", s3);

    let hello = "Здрав";

    let s = &hello[0..6];
    // let s = &hello[0..5]; // this panics because no clear char boundary :o

    println!("{}", s);

    let clear = "abc";
    let s = &clear[0..3]; // I suppose this works because every character is only stored as one byte ...
                          // avoid doing slices without handling the panic (weird how rust allows that though without using unsafe)
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }

    // hash maps

    let mut scores_a = HashMap::new();

    scores_a.insert(String::from("blue"), 10);
    scores_a.insert(String::from("red"), 50);

    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores: Vec<i32> = vec![1, 5];

    // This builds a HashMap<&String, &i32>
    // Not like in the example which needs to be HashMap<String, i32> not sure yet why ... i'll need to take a look at iter, zip and collect interaction later
    let scores_hash: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores_hash);

    println!("{:?}", scores_hash.get(&String::from("blue")));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 1);

    scores.entry(String::from("Yellow")).or_insert(5);
    scores.entry(String::from("Blue")).or_insert(5);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
