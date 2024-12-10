// fn main() {
//     let m=1;
//     if m==1 {
//         println!("This is One");
//     }else{
//         println!("This is other than One");
//     }
// }


//Assignment of a variable to if the variable
// fn main() {
//     let m=1;
//     let y:&str=if m==1 { "six" } else { "one" };
//     println!("{y}");
// }


//Loop

// fn main() {
//     loop {
//         println!("Mahesh");
//     }
// }

//Name the loop and return the value

// fn main() {
//     let mut m=0;
//     let result = loop{
//         m += 1;
//         if m==10{
//             break m*2;
//         }
//     };
//     println!("{}", result);
// }


//condition loop

//while

// fn main() {
//     let mut m=10;
//     while m !=0{
//         println!("{m}");
//         m -=1;
//     }
// }

//Looping through a collection with for

// fn main() {
//     let a=[3;5];
//     let mut index=0;

//     while index<a.len(){
//         println!("{}",a[index]);
//         index += 1;
//     }
// }


//using for loop

fn main() {
    let a=[3;5];
    for i in a{
        println!("{}",a[i]);
    }
}   