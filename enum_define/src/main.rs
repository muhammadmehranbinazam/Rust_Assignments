#[derive(Debug)]
enum Student{
    online,
    offline,
}
fn main() {
    let mehran=Student::online;
    println!("{:#?}",mehran);
}
