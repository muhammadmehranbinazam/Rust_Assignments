#[derive(Debug)]
enum Student{
    online,
    offline,
}
impl Student{
    fn call(&self){
        println!("{:#?}",self);
    }

}
fn main() {
    let mehran=Student::online;
    mehran.call();
}
