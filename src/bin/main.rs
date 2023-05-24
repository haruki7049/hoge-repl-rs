use hoge_repl_rs::repl::*;

fn main() {
    const isLooped: bool = true;

    while isLooped == true {
        read();
        eval();
        print();
    }
}
