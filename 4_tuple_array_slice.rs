// Some things implement the Debug traint so we have to use :? to print them. For example, run time strings, array, vectors etc
// Default iteration is [a..b] it means from index a (included) till b (excluded). Ex: 1..5 gives 1,2,3,4

//IMPORTANT NOTE: Look at the slicing function closely, specially from line 91 to understand the concepts of Borrowing and Scoping

fn array_example(){
    // You can increase or decrease the size of array AND they'll have only one type of data inside them unlike Python lists
    // it means there will be 5 elsments, each of integer. You can't have mixed types inside array like in Python list
    let x: [i32; 5] = [0,0,0,0,0]; // Same thing as doing let x: [0,0,0,0,0]
    println!("X: {:?}", x); // Look closely the {:?}. This is how you print the whole Array, String etc

    // if you do x[3] = 3 it'll give error as it's immutable. So you have to make it mutable
    let mut y = [1; 5]; // Same as doing x = [1,1,1,1,1]
    y[3] = 3; // you can't do y[3] = 3.1 becaue it expects the integers
    println!("Y: {:?}", y);

    let z = ["Hi", "there!","Hello", "World"]; // You can have strings but one array will have only one type of elements
    
    // Iterating and accessing elements in an array using index
    for index in 0..z.len(){  // Same as doing like: for index in 0..=3
        println!("Value at inex {} is {}", index, z[index]); // This is how you access elements
    }

    // Directly access the values of array
    for value in z.iter(){
        println!("{}", value); // This is how you access elements
    }

    // Access index as well as Values just like Python enumerate()
    for (index, value) in z.iter().enumerate(){
        println!("index: {} | value: {}", index, value); // This is how you access elements

    }

}

fn tuple_example(){
    // They can have different type of data and elements are accessed differently than array
    let mut tup: (&str, i32, [&str; 2], f64) = ("Hello", 3, ["World", "Order"], 4.3); // Tuple having mix types like str, int, array, floatarray
    // Same as doing: let mut tup: (&str, i32, [&str; 2], f64) = ("Hello", 3, ["World", "Order"], 4.3);
    
    // Accessing elements. Unlike Python tuples, if you make them mutable, you can change values
    tup.2[0] = "Earth"; // Look how elements are accessed. Also, we just changed the array value inside tuple

    // De-structuring tuples like Python list and tuples
    let (str1, int1, arr1, float1) = tup;

    println!("Values of variables in tuple are:  {}, {}, {:?}, {}", str1 , int1 , arr1, float1); 
    //Look at the third print as we are printing full array, we have used {:?}
}

fn vector_example(){
    // It is almost like Python list. MEans you can add, remove values from it. Only difference is that the datsa type should be of same type like array
    let mut vect: Vec<i32> = vec![0, 1,  10]; // It'll only have integers. If no data type is defined, it'll be inferred

    let empty_vect: Vec<&str> = Vec::new(); // it means that create emty vector which is supposed to have strings in it

    vect.push(100); // Added a new element. Now it has 4 elements. It must be mutable for addition and removal
    vect.remove(0); // Removed the value at 0th index. Now it has again 3 values

    vect[2] = -1; // Changinf the value. Vector must be mutable
    
    //Accessing elements can be done by the [] notation or the .get() method
    // Notice the Difference between .get() and [] methods of accesing. Also, for the .get(), we use {:?} for printing
    println!("Accessing Values of a Vector using vect.get(2) gives {:?} while using vect[2] gives {}", vect.get(2), vect[2]);
}

fn slice_example(){
    // Slices are, well, literally "slices" which basically means a Contiguous subset of an iterator (Array, Vector, Strings)
    let mut arr = [1,2,3,4,5,6,7,8,9,0]; // immutable
    
    // look at the "&". This is related to giving "ownership"
    let slice_arr = &arr[2..5]; // Create a Subarray starting from Index 2 (inclusive) till Index 5 (excluded)

    let slice_from_start = &arr[..5]; // It mean start from 0 (included) index till 5 (excluded)
    let slice_till_end = &arr[4..]; // it means the slice now will start at 4 (included) and will go till END
    
    // You can not edit the mutable slice
    println!("Sliced array starting from 2-5 is {:?} || From START-5 is {:?} || From 4-END is {:?}", slice_arr, slice_from_start, slice_till_end);

    // Making slices mutable and changing values
    let vector: Vec<i16> = vec![1,2,3,4,5,6,7,8,9]; //immutable vector
    let mut mut_arr = [1,2,3,4,5,6,7,8,9,0]; // mutable array

    let mut slice_1 = &arr[..5];
    // slice_1[0] = -1; It'll produce error even though the Array is mutable and we have declared the slice as mutable
    
    let mut slice_2 = &vector[..5];
    // slice_2[0] = -1; this will also give error

    let mutable_slice = &mut arr[..5]; // Notice we did not use "let mut" here but on the right hand side "&mut"
    // let slice_3 = &mut vector[..5];  //Can't be declared because the parent is immutable

    mutable_slice[0] = -1;

    println!("Mutable and changed slice: {:?}: ", mutable_slice); 

    println!("{:?}", arr); // Look -> Original value is changed itself in the Original Array

    // IMPORTANT NOTE: println!("{:?}, {:?}", mutable_slice, arr);  gives error but on seperate lines it does not
    // Also, printing mutable_slice AFTER printing arr will give error due to Borrow rules
    // This can be solved by SCOPING. Which means you put Line 91-96 within {}

}

fn string_example(){
    // Strings are of mainly 2 types. String (Read + Write MAnipulation) , &str (Read only)
    // All the functions are here: https://doc.rust-lang.org/std/string/struct.String.html
    // Reference Youtube video: https://www.youtube.com/watch?v=CpvzeyzgQdw

    let mut str1: &str = "Mutable + Unmodifiable"; // even though it is mutable but it's characters can't be changed
    str1 = "Still Mutable and still not modifiabe";
    // str1[0]; 's'; // Can't be done

    let mut str2: String =  String::from("Mutable");
    // str2 = "Mutable + Modifiable"; // This will give error as new string is of type &str
    str2 = "Mutable + Modifiable".to_string(); // Now it is if type String instead of &str

    let mut empty_str = String::new(); // Empty String
    empty_str.push('A'); // Push single Character
    empty_str.push_str("dded String"); // Added to the existing String
    let slice = &empty_str[0..5]; // Note it is borrowed operation

    println!("Slice: {:?}", slice);

    let mut replaced = empty_str.replace("Added", "Inserted"); // Returns a new string by Replacing sub string
    println!("Original: {} || Replaced: {}", empty_str, replaced);

    replaced.replace_range(9.., "new string"); // In PLACE Operation From 9th index to end of string replace
    println!("'replaced' now has become: {}", replaced); // See, you don't need {:?} for String type but you do for &str


    for ch in replaced.chars(){
        println!("{}", ch);
    }
}

fn main(){
    array_example();
    tuple_example();
    vector_example();
    slice_example();
    string_example();
}
