/// this module provides common tests.
#[cfg(test)]
mod tests{
    use crate::repl;

    /// read function test.
    #[test]
    fn read_test(){
        repl::read();
    }
}
