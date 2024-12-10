//Functions

// fn main(){
//     another_function();
// }

// fn another_function(){
//     println!("Another function");
// }

// fn main(){
//     let y={
//         let x=3;
//         x+1
//     };
//     println!("{y}");
// }

//function return value

// fn five() -> i32{
//     5
// }

// fn main(){
//     let y:i32=five();
//     println!("{y}");
// }

//function parameter return value

fn five_plue(value:i32)->i32{
    5+value
}

fn main() {
    let y:i32=five_plue(32);
    println!("{y}");
}