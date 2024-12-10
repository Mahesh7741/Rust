//References

// fn main() {
//     let str=String::from("hello world");
//     let (strs,length)=calculate_length(str)
//     println!("{strs}{length}");
// }

// fn calculate_length(s:String) ->(String,i32) {
//     let len=s.len();
//     (s,len)
// }


// fn main() {
//     let s=String::from("hello world");
//     let len=calculate_length(&s);
//     println!("{}",len);
// }
// fn calculate_length(s:&String) ->usize {
//     s.len()
// }

// fn main() {
//     let mut s=String::from("Mahesh");
//     add_sername(&mut s);
//     println!("{}",s);
// }
// fn add_sername(name:&mut String) {
//     name.push_str(" Savant");
// }

// fn main() {
//     let s;
//     {
//         let x=42;
//         s=&x;
//     }
//     println!("{s}");
// }

//slice Type

// fn main(){
//     let array=[1,2,3,4,5];
//     let silce=&array[1..4];
//     println!("{:?}",silce);
// }

fn main(){
    let mut array=[1,2,3,4,5];
    let silce=&mut array[1..4];
    silce[0]=10;
    println!("{:?}",array)
}

