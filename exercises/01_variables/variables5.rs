fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // in rust we can shadow variables https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
