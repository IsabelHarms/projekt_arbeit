pub fn make_exp(mut s: &str)  //return
{
    while s.starts_with(" ") { s = &s[1..]; }// trim leading blanks
    while s.ends_with(" ") {s = &s[..s.len()-1]; } // trim trailing blanks
    if s.starts_with("(") && s.ends_with(")") { s = &s[1..s.len()-1]; } // discard outer (..)
    println!("'{}'",s); // for debugging only
    let split = outer_plus(s);
    if split < s.len()
    { 
        make_exp(&s[..split]);
        make_exp(&s[(split+1)..]);
    }
    //Plus { left:  exp, right: make_exp()}
}

fn outer_plus(s: &str)-> usize // cannot get slices from type u16 expressions?
{
    let mut depth: i16 = 0; //unsigned?
    for i in 0..s.len()-1
    {
        let c = s.chars().nth(i);
        match c
        {
            Some('(') => depth=depth+1,
            Some(')') => depth=depth-1,
            Some('+') if (depth == 0) => return i,
            _other => depth=depth,  //ugly

        }
    }
    //if depth is not 0 here, the given expression was invalid
    s.len() //No outer plus found
}

