#[derive(Debug)]
struct Student{
    name : String,
    number:u32,
}

fn main() {
    let mehran=Student{
        name:String::from("Mehran"),
        number:12,
    };

    println!("{:#?}",mehran.name);
}
