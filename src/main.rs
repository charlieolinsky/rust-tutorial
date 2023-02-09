fn main() {

    //VARIABLES
        let x = 4;
        println!("x is: {}", x);

            //NAME SHADOWING
            {
                let x = x - 2;
                println!("x is: {}", x)
            }

        let x = x + 1;
        println!("x is: {}", x);

        //TYPE CHANGE
            let x = "Hello!"; 
            println!("x is: {}", x);

        //CONSTANTS
            //must be written in capital snake case, and have a defined type. 
            const SECONDS_IN_MINUTE: u32 = 60;
            println!("constant is: {}", SECONDS_IN_MINUTE);



    //SCALAR DATA TYPES
        
        //INTEGERS
            //Integers w Bit specificatons. 
            // i8     u8
            // i16    u16
            // i32    u32
            // i64    u64
            // i128   u128
            //NOTE: range of values, k, is a funtion of the # of bits, n 

            //Signed Integer
            let k: i8; 
            // k = [(-2^7), (2^7)-1] = [-128, 127]

            
            //Unsigned Integer
            let k: i8; 
            // k  =  [0, (2^n)-1]  =>  [0, (2^8)-1]  =  [0, 255]

        //FLOATING-POINT
            //Floating point w bit specifications.
            // i32  i64

            let fp: f32 = 10.8; //Single-precision
            let fp: f64 = 10.8; //Double-precision

        //BOOLEAN
            let f: bool = false; //0
            let f: bool = true;  //1

        //CHARACTER
            let letter: char = 'a';

            
    //COMPOUND TYPES


}
