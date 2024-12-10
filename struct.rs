//Struct or StructType
struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

fn main(){
    let user1=User{
        username:String::from("mahesh"),
        email:String::from("mahesh@gmail.com"),
        sign_in_count:125,
        active:false,
    };
    println!("{:#?}", user1);
}