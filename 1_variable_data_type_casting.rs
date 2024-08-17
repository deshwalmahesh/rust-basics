// This is how you write Comments
// main() is the entry point for the code

fn show_variable_property(){
    // u8, i16, f32, char, &str etc etc are data types. You can provide them or the program will automatically infer these too

    let immut_var = 15; // This is Immutable Variable means it's value can't be changed. By default every variable is immutable
    // immutVar = 45; This will Give Error as you can't change it's value
    let immut_var = "Hello World"; // You can assume that this this is a new Variable or Memory address so you can change the data types too

    let mut mut_var: i32 = 32; // This is Mutable variable it means you can change it's value. We are explicitely telling that it's of type "i32"
    mut_var = 52; // This works as long as the data type is same. It's the same Variable and same Memory
    // mutVar = "This will give error as you can not change data types";

    const CONSTANT : u32 = 30; // It is a "constant" means it's value can't be changed, ever. Variables names are snake_cased but constants are UPPER_CASED
    // let MY_AGE = 30; // Tjis can't be done because you just can not Shadow (Override) the constants
    

    println!("This is how you print variables -> mutVar: {} || immutVar: {} || CONSTANT: {}", mut_var, immut_var, CONSTANT);
}


fn simple_data_types(){
    let mut integer: i128 = 4; // Different types of integers like unsigned (u8,u32 ..) and signed (i8, i32, ..)
    // let mut integer: u8  = 123456789; This will give error as the range of Unsigned Integer is way too less than this number and it's overflowing
    // let integer: u128 = -300; Error as Unsigned can't have negative values
    let mut float: f32 = -3.0;
    let mut character: char = '*'; // using double quote "*" would mean that it is a string (&str)
    let mut flag: bool = true; // true , false

}

fn type_casting(){
    // Change one data type to another by using  VAR_NAME as DATA_TYPE
    let flag = true;
    let integer = flag as i32;

    // let mut ascii_character = integer as char; Error as only 'u8' are allowedfor conversion
    let u8_int: u8 = 64;
    let mut ascii_character = u8_int as char;
    // let mut string = integer as &str; It has to be done via the format! macro or integer.to_string(); method
}


fn main(){

    let same_func = show_variable_property; // You can save the function in a variable too
    same_func(); // This is how you call a function. Could also be done directly as show_variable_property()
    
    let simple_string =  "string without explicitely declsring data type"; // look below on how to declare data type of string 
    let multi_line_string: &str =  "line 1
    line 2\nline 3 with backslash n
line 4 with\t a tab
    line 5 after newline and tab"; // This was multi line string similar to fstring in Python

    function_below_main(); // Calling the Function like this
}


fn function_below_main(){
    println!("This ia function below main(). A function can be anywhere. Above or Below the main()");
    // No Return is needed
}
