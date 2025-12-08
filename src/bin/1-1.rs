fn main() {
    // Basic variable syntax
    let var = 5;
    println!("var: {}", var);
    // If you want a mutable variable then you need to use mut
    let mut var2 = 10;
    println!("var2: {}", var2);
    var2 = 12;
    println!("var2: {}", var2);
    // if you ever need to declare a variable or you need something for
    // a function call (more about that later), use an underscore at the start
    // so the compiler stops complaining at you
    let _never_used = "hehe";

    // You may be wondering why println has a ! at the end
    // if you are its because this indicates that it is a macro
    // and not a function. For now all you need to know is that
    // a macro will accept a variable amount of parameters whereas
    // a function must declare the number and type of parameters.
    // If you want to learn more you can always read the rust docs
    // https://doc.rust-lang.org/book/ch20-05-macros.html
    println!("hee {}", foo("hee"));
    println!("hee {}", foo("haa"));
}

// This function foo takes a reference to a string and returns
// a String. In the next file we will explore why this matters
fn foo(example: &str) -> String {
    if example == "hee" {
        return "haa".to_string();
    }
    return "idk".to_string();
}
