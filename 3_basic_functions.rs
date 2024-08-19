// NO "DEFAULT" params and Function Overloading is supported by Rust? What? Really? 
// Check: https://users.rust-lang.org/t/default-and-optional-parameter/27693
// We also need to define the output type. Leaving it blank means () type

fn basic_working(inp: i32,) -> i32{ // Input is i32 and output is i32 too
    // inp += 50; It won't work as it is Immutable
    inp +  50 // If there is no semicolon, it'll return the value. Same as below 
    // return inp + 50 // Same as above
}

fn mutable_input(mut inp: i32) -> i32{ // Notice the "mut" now
    inp += 100; // This is mutable means it's not the same variable and is a copy of it
    return inp // Can be done as simple inp without semicolon like above
}

fn multiple_return(x:isize, y:isize) -> (isize, isize){
    (x + y, x - y)
}

// This is the best I could reach to function overloading and Default parameters
fn optional_param(x: i32, z: Option<&str>) -> ((i32, Option<&str>), String) {
    // If Z is noe, return X only. If it is a string, convert X to string and add to it. If Z is a number, add two numbers and return
    match z {
        None => ((x, None), (x).to_string()), // if it is None
        Some(z_val) => {
            if z_val.parse::<i32>().is_ok() { // if it is a parsable number
                let z_int = z_val.parse::<i32>().unwrap();
                ((x, Some(z_val)), (x * z_int).to_string())
            } else { // if it is a complete string
                ((x, Some(z_val)), format!("{}{}", x, z_val))
            }
        }
    }
}


fn main(){
    let x: i32 = 1;
    let y = basic_working(x);
    let z = mutable_input(x); // Look closely, our "x" is not mutable but the function definition makes it mutable
    println!("{}, {}, {}", z, y, x);

    let (a,b) = multiple_return(1, 1);
    println!("{}, {}", a,b);

    // Optional Parameters. A bit complicated
    let result1 = optional_param(5, None); // like None in python
    println!("{:?}", result1);

    let result2 = optional_param(5, Some("2"));
    println!("{:?}", result2);

    let result3 = optional_param(5,  Some("hello"));
    println!("{:?}", result3);
}
