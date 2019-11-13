fn not_five() {
    5; // notice semi-colon : expression => statement, no return.
}

fn another_five() -> i32 {
    return 5;
}

fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    let y = another_five();
    let z = not_five();

    println!("{}", x);
    println!("{}", y);
    // println!("{}", z); you can't, this is ()
}
