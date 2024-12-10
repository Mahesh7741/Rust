//Tuples

//First method 
//Destructuring
// fn main() {
//     let tup:(u8,bool,f32)=(255,true,20.0);
//     let (a,b,c)=tup;
//     println!("{}",a);
//     println!("{}",b);
//     println!("{}",c);
// }


//Second method

fn main() {
    let tup:(u8,bool,f32)=(255,true,20.0);
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);
}