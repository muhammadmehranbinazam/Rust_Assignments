#[derive(Debug)]
enum Option<T>{
    some(T),
    none,
}

fn main() {
    let number=Option::some(13);
    let st=Option::some(String::from("Mehran"));
    // let n:Option<T>=Option::none;
    println!("{:#?}",number);
    println!("{:#?}",st);
    // println!("{:#?}",n);

}
