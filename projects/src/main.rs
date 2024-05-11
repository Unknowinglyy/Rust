fn main() {
    let x = 5;

    println!("x = {x}");
    let x = 6;

    println!("x = {x}");

    let y = 57u8;

    println!("y = {y}");

    let five = five();
    let six = six();
    println!("the value of five() is {five}");

    println!("the value of six() is {six}");
}


fn five() -> i32 {
    5
}


fn six() -> i32{
    return 6;
}
