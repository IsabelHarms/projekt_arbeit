mod exp;
fn main() {
let no_of_tests = 9;
let tests = ["(2+3) * (1+4)+5 + 8*2","((3+2))","83+2","+1+(+2*(+4)+3)+(32)","",")","12345$234","12+(23*4","3+**4","4*3+)5"];
let expected = ["46","5","85","44","Fehler","Fehler","Fehler","Fehler","Fehler","Fehler"];
    for t in 0..=no_of_tests {
        println!();
        println!("Test No.{}:", t+1);
        exp::run(tests[t]);
        println!("Expected: {}",expected[t]);
    }
}