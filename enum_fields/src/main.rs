#[derive(Debug)]
enum mesg{
    quit,
    write(String),
    color(u32,u32,u32),
    location{x:u32, y:u32}
}


fn main() {
    let q=mesg::quit;
    let w=mesg::write(String::from("Hello world"));
    let c=mesg::color(1,23,23);
    let l=mesg::location{x:121,y:232};
    println!("{:#?}",q);
    println!("{:#?}",w);
    println!("{:#?}",c);
    println!("{:#?}",l);


}
