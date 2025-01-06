pub fn tuple() {
    let data: (i32, f64, bool) = (10, 1.5, true);

    println!("{:?}", data)
}

#[allow(dead_code)]
pub fn tuple_params() -> (i32, f64, bool) {
    let data: (i32, f64, bool) = (10, 1.5, true);
    data
}

#[allow(dead_code)]
pub fn read_tuple() {
    let data = tuple_params();

    println!("{} {} {}", data.1, data.0, data.2);
}
