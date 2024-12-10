// fn main(){
//     let mut num:u8=45;
//     println!("{}",num);
//     num=2;
//     println!("{}",num);

// }

//Constants

// fn main(){
//     const MY_NAME: &str ="Mahesh";
//     println!("{}",MY_NAME);
// }

//Shadowing

fn main(){
    let x=12;
    let x=x+1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}