pub trait Exp { 
    fn eval(&self) -> i32; 
} 
 
pub struct Plus<T:Exp> { 
    pub left: T, 
    pub right: T 
}

pub struct Mult<T:Exp> {
    pub left: T,
    pub right: T
}
 
struct PlusN<T:Exp> { 
   operands : Vec<T> 
} 
 
 
pub struct Int { 
    pub val: i32 
} 
 
impl Exp for Int { 
    fn eval(&self) -> i32 { 
        return self.val 
    } 
} 
 
impl<T:Exp> Exp for Plus<T> { 
    fn eval(&self) -> i32 { 
      return self.left.eval() + self.right.eval() 
    } 
} 

impl<T:Exp> Exp for Mult<T> {
    fn eval(&self) -> i32 {
        return self.left.eval() * self.right.eval()
    }
}
 
 
pub fn run() { 
    let e = Mult { left: Int{val:1}, right: Int{val:2}}; //3+1*2
 
    println!("{} done",e.eval());  
}