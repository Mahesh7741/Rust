// fn main() {
//     let mut str=String::new();
//     str.push_str("Mahesh");
//     println!("{}", str.to_lowercase());
// }

fn main() {
    let s=String::from("Hello");
    take_ownership(s);

    let x=5;
    make_copy(x);
}

fn take_ownership(s:String){
    println!("{}", s);
}

fn  make_copy(x:i32){
    println!("{}", x);
}