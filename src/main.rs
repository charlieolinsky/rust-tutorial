fn main() {

    //Variables
    let x = 4;
    println!("x is: {}", x);

    //Name Shadowing - create a new scope
    {
        let x = x - 2;
        println!("x is: {}", x)
    }

    let x = x + 1;
    println!("x is: {}", x);

    //Type changing
    let x = "Hello!"; 
    println!("x is: {}", x);

    //Constants
    //must be written in capital snake case, and have a defined type. 
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("constant is: {}", SECONDS_IN_MINUTE);


}
