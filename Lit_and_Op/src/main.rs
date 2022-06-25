pub fn main() {
    println!("Hello, world!");
    // Intiger addition 
    println!("1 + 2 = {}", 1 + 2);
    println!("1 + 2 = {}", 1u32 + 2);

    //Int subtraction 
    println!("1-2 = {}", 1i32 - 2);
    println!("1-2 = {}", 5i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    //<
    println!("1 < 5 is {}", 1u32 < 5);

    /*
    Use underscores to improve readability!
    */
    println!("One million is written as {}", 1_000_000u32);

}
