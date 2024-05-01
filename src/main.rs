//variables in rust
// fn main() {
//     let x= 1;
//     println!("The value of x is: {x}");
// }
//----------------------------------------------------------

//Booleans in Rust

// fn main(){
//     let is_male  = true;
//     let is_above_18 = true;

//     if is_male{
//         println!("you are male")
//     }else{
//         println!("You are not male")
//     }

//     if is_male && is_above_18{
//         print!("you are a legal male")
//     }
// }

//----------------------------------------------------------
//mutable
// fn main(){
//     let mut x = 10;
//     let i = 0;
//     for i in 0..100{
//         x = x+10;
//     }
//     println!("x={}",x);
// }
//----------------------------------------------------------

//Strings
// fn main(){
//     let greeting = String::from("Hello World");
//     println!("{}",greeting);

//     let char1 = greeting.chars().nth(100);
//     // print!("char1:{}",char1.unwrap());
//     //OR

//     match char1{
//         Some(c) => print!("{}",c),
//         None => print!("No character at index 100")
//     }
// }

//----------------------------------------------------------

//Conditions and Loops

fn main(){
    for i in 0..11{
        print!("{} ",i);
    }
}