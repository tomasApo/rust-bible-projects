fn main() {

    let x = 5;
    let x = x + 1;

    let spaces = "   ";
    let _spaces = spaces.len();

    println!("spaces {}",_spaces);
    
    
    {
    
        let x = x * 2;
        println!("value of x inside scope:{}", x);

    }
    
    println!("The value of x is: {}",x);

}