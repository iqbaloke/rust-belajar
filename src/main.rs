mod variable;

fn main() {
    println!("Hello, world!");
    variable::helper::greet("Alice");

}

#[test]
fn hello_test(){
    println!("Hello, world");
}