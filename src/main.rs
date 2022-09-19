fn main() {
    let mut counter = 0; //We are creating a variable with a scope within main function. This variable's value can change but memory address remains the same

    counter = loop{                 // We are creating a loop 
        counter = counter + 1;      //We are incrementing the value assigned to the variable by 1
        println!("{}", counter);    //We are printing the current value for the variable
        if counter == 10{           //Checking the current value of the variable
            break counter *  2;     //Breaking the loop
        }
    };
    println!("The result is {}", counter); //Printing the current value of the variable after the last condition within the loop scope
}
