pub fn number() {
    let a: i32 = 10;
    println!("{}", a);

    let b: f64 = 20.1;
    println!("{}", b);
}

#[allow(dead_code)]
pub fn conversion_fun(){
    let a: i8 = 10;
    print!("a{}", a);

    let b: i16 = a as i16;
    println!("b{}", b);

    let c: i32 = a as i32;
    println!("c{}", c);


}
