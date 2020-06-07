#[derive(Debug)]
struct Student{
    name :String,
    age:u32,
    percentage:f64,
    grade:String,
}
impl Student{
    fn return_instance()->Student{

        let result=Student{
            name:String::from("Mehran"),
            age:40,
            percentage:65.8,
            grade:String::from("A"),
        };
        result
    }
    fn print_percentage(&self){
        println!("The percentage is {}",self.percentage);
    }
}

fn main(){
    
    let insitance=Student::return_instance();
    insitance.print_percentage()
    
    
   // println!("{:#?}",result);
}