/*  To get input from the user and other stuff
 *
 * */
#![allow(unused)]
 use std::io::*;

 fn main() {
     //User input variable
     let mut user_in = String::new();
     let mut user_2 = String::new(); 
     println!("Enter you name: ");
     stdin().read_line(&mut user_in).expect("Use the alphabet only jerk!!...Mission failed");
 
     println!("Hello {}",user_in);

     println!("Enter a number between 1 - 5: "); 
     stdin().read_line(&mut user_2).expect("Failure to input a character jerk!!...Mission failed"); 
     
     //convert the string into an int
     let numb_in = user_2.trim();
    match numb_in.parse::<u32>(){
        Ok(i) => println!("Your entry: {}", i),
        Err(..) => println!("This in not what I asked {}", numb_in),

    };
    //This is a silly loop. There has a be a better way with match (switch/case)t      
    let conv = numb_in.parse::<u32>();
    
        //if (conv >= Ok(6)) || (conv <= Ok(0))  {println!("You are a moron."); }
        if conv == Ok(1) {println!("One for the msney!");  }
        if conv == Ok(2) {println!("Two for the show!!"); }
        if conv == Ok(3) {println!("Three to get ready!!!"); }
        if conv == Ok(4) {println!("Four...Lets GO!!!!"); }
        if conv == Ok(5) {println!("Stayin alive until five!!!!!"); }
        
        
 }

