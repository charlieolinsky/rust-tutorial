fn dataTypes() {
    
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
            //We can overwrite existing variables with a new type: 
            let x = "Hello!"; 
            println!("x is: {}", x);

            //Notice that a variable set to another variable will 
            //retain the original variables type
            //and not default to i32:  
            let z: u8 = 5; 
            let y = z; 

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
            let _k: i8; 
            // k = [(-2^7), (2^7)-1] = [-128, 127]

            
            //Unsigned Integer
            let _k: i8; 
            // k  =  [0, (2^n)-1]  =>  [0, (2^8)-1]  =  [0, 255]

        //FLOATING-POINT
            //Floating point w bit specifications.
            // i32  i64

            let _fp: f32 = 10.8; //Single-precision
            let _fp: f64 = 10.8; //Double-precision

        //BOOLEAN
            let _f: bool = false; //0
            let _f: bool = true;  //1

        //CHARACTER
            let _letter: char = 'a';

            
    //COMPOUND TYPES

            //TUPLE
                //you can place any data types in here
                let mut tup: (i32, bool, char) = (1, true, 's'); 

                //PRINTING
                    //println!("{}", tup);
                    //^ Error: must reference by index
                    println!("Tuple: {}", tup.0); 

                //MUTATION (must include mut) 
                    tup.0 = 10; 
                    println!("New Tuple: ({}, {}, {})", tup.0, tup.1, tup.2);

                    //you can actually mutate the whole thing:
                    tup = (3, false, 'a'); 
                    println!("New Tuple: ({}, {}, {})", tup.0, tup.1, tup.2);

                //NOTE: You can not change the size of a tuple without reintializing it. 
                //tup = (3, false, 'a', 200); 
                //^ERROR

            //ARRAY
                let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

                //Arrays, unlike tuples, must contain all of the same type 
                //of element. No mismatching types. 

                //PRINTING
                    //println!("{}", arr);
                    //^ Error: must reference by index
                    println!("New Array: [{}, {}, {} ...]", arr[0], arr[1], arr[2]);

                //NOTE: Arrays have no default values like 0. 
                //An array declared with no values will be empty. 


}