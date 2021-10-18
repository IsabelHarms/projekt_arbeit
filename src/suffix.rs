pub fn run()
{
    let examples: [&str;4] = [":::","",": oh, ::yes : we : : can","but don't use Rust!"];
    for s in examples {
        println!("Suffix of '{}' = '{}'",s, extract(s));
    }
}

fn extract(s:&str) -> &str
{
    let mut index = s.len();
    let mut count = 0;
    while index > 0
    {
        index -=1;
    
        let c = s.chars().nth(index);
    
         if c == Some(':')
         {
            count+=1;
            if count == 2 { return &s[index+1..]; }  //+2 for other variation
             
         }
         else 
         {
             count = 0;
         }
    }
    s
}