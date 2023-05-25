use hoge_repl_rs::repl;

fn main() {
    let isLooped: bool = true;

    // loop. should do stop by change inLooped variable in anywhere.
    while isLooped == true {
        let input = repl::read();
        let output: &str = repl::eval(input);
        repl::print(output);
    }

    println!("Bye.");
}
