use calc::codegen::compile::compile_string;
use std::env::args;

fn main() -> anyhow::Result<()> {
    let input = args().nth(1).expect("Failed to find argument.");

    println!("problem is {}", input);

    println!("The answer is {}", compile_string(&input)?);

    Ok(())
}
