#[derive(Debug)]
enum Student{
    online,
    offline,
}
fn call(X:Student){
    println!("{:#?}",X);
}

fn main() {
    let m=Student::offline;
    call(m);
}
