fn main() {

    let mut divisor: f64 = 1.0;
    let mut result: f64 = 0.0;

    for i in 0..1_000_000_000i64 {
    
        let sub_result: f64 = 4.0 / divisor;
    
       if i % 2 == 0 {
            result = result + sub_result;
        }
        else {
            result = result - sub_result;
        }

        divisor = divisor + 2.0;
    }
    println!("{:.50}", result);
}
