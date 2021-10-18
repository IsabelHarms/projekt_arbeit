pub fn run()
{
    let examples: [&str;7] = ["abcdef","","nebel","desserts","aibohphobia","123..abc...?","wow"];
    for s in examples {
        println!("Anadrome of '{}' = '{}'",s, reverse(s));
    }
}

fn reverse(s: &str) -> String {
    let characters = s.chars();
    let mut reverse = String::from("");
    for c in characters
    {
        reverse = format!("{}{}", c,reverse);
    }
    reverse 
}