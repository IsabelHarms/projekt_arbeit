mod exp2;
fn main() {
let no_of_tests = 4;
let tests = ["(2+3) * (1+4)+5 + 8*2","((3+2))","83+2",")","1000 * a"];
let expected = ["46","5","85","-","-"];
    for t in 0..=no_of_tests {
        println!();
        println!("Test No.{}:", t+1);
        exp2::run(tests[t]);
        println!("Expected: {}",expected[t]);
    }
}