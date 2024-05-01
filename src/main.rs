//variables in rust
// fn main() {
//     let x= 1;
//     println!("The value of x is: {x}");
// }


//Booleans in Rust

fn main(){
    let is_male  = true;
    let is_above_18 = true;

    if is_male{
        println!("you are male")
    }else{
        println!("You are not male")
    }

    if is_male && is_above_18{
        print!("you are a legal male")
    }
}