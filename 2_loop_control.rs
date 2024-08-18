// Show Working of Loops (with continue and break) and Control (if-else and match) Statement

fn for_loop(){
    // Loop from 1 to 5 (EXCLUDE "5")
    for i in 1..5{
        println!("Value of i is: {}", i);
    }

    // Using "="" will INCLUDE "5" in the iteration
    for j in 1..=5{
        println!("Value of J is: {}", j);
    }

    // This will reverse the loop from 5 (including) to 1 (including)
    for k in (1..=5).rev(){ 
        println!("Value of K is: {}", k);
    }


    for l in 1..=5{
        if l == 2 {continue;} // Skip the iteration at 3
        else if  l == 4 {break;} // Break the loop at 4
        println!("Value of L is: {}", l);
    }

}

fn infinite_loop(){
    // similar as while True but we use the word 'loop' here
    let mut x = 0;
    loop {
        if x == 5 {break;}
        x += 1;

        println!("Value of X in Infinite Loop is: {}", x);
    }
}

fn while_loop(){
    let mut y = 0;
    while y < 5{
        println!("Value of Y in while Loop is: {}", y);
        y += 1;
    }
}

fn if_else(){
    // Just like every other programming. Mostly like C or C++
    // Uses if, else if, else
    for a in 1..5{
        if a == 1 {
            println!("This is 'Un' in Spanish");
        }
        else if a == 3 {
            println!("This is 'Teen' in Hindi");
        }
        else {
            println!("Number is: {}", a);
        }
    }
}

fn match_control(my_string: &str){
    // It's just like the 'CASE' statement
    match  my_string{
        "Hello" => println!("Hello World"), // Mind the comma instead of simicolon
        "Hi" => println!("Hi, Good Morning"),
        _ => println!("You got something to say? Blah! Not interested") // '_' underscore is a wildcard for any other value or a default  
    }
}

fn main(){
    for_loop();
    infinite_loop();
    while_loop();
    if_else();

    match_control("Hello");
    match_control("How are you?");

}
