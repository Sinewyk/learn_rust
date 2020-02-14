fn main() {
    let opt: Option<i32> = None;

    if let Some(val) = opt {
        println!("{}", val);
    }

    let val_or_zero = match opt {
        Some(val) => val,
        None => 0,
    };
    println!("{}", val_or_zero);
}
