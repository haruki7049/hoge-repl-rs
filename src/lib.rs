mod tests;

/// this module provides methods of repl.
pub mod repl {
    use std::io;

    /// read from hoge prompt.
    pub fn read() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    /// evaluate input sentence.  
    /// TODO:
    /// 1. 
    pub fn eval(input: String) -> str{
    }

    /// print the result.
    /// TODO:
    /// 1. 
    pub fn print(){
    }
}
