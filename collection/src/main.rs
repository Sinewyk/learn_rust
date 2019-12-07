fn main() {
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
}
