// fn main() {
//     let int_type = 1; 
//     let string_type = "Mahesh"; 
//     let float_type: f32 = 4.5; 
//     let bool_type = true; 
    
//     println!("Integer: {}", int_type);
//     println!("String: {}", string_type);
//     println!("Float: {}", float_type);
//     println!("Boolean: {}", bool_type);
// }

//Integer

//signed int
// fn main(){
//     let num:i8=-10;
//     println!("{}",num);
// }

//Unsigned int
// fn main(){
//     let num:u8=10;
//     println!("{}",num);
// }

//Integer overflow 
// fn main() {
//     let age:u8 = 255;
 
//     // 0 to 255 only allowed for u8
//     let weight:u8 = 256;   //overflow value is 0
//     let height:u8 = 257;   //overflow value is 1
//     let score:u8 = 258;    //overflow value is 2
 
//     println!("age is {} ",age);
//     println!("weight is {}",weight);
//     println!("height is {}",height);
//     println!("score is {}",score);
//  }


//floating point number

//  fn main() {
//     let num=2.0;
//     let num2:f32 =20.0;
//     println!("{}",num);
//     println!("{}",num2);
//  }

//Component Type

fn main() {
    let tup:(i32,u8,bool)=(2100,255,true);
    let (x,y,z)=tup;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
}