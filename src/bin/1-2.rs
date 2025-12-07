fn main() {
    // The main difficulty I faced when learning rust were
    // the rules surrounding Ownership. You might wonder
    // if rust handles memory for me, why do I need to know the
    // rules? Its because if you are like me you will inevitably
    // try to use a variable that's already lost ownership.

    // The simplest concept is that variables will exist only as long as
    // they are in scope
    {
        let _x = 10;
    }
    // at this point x is no longer accessible as the memory
    // has been freed once the scope it was in ended.
    
    // The more interesting thing is that this will prevent
    // dangling pointers. If you uncomment the first foo function
    // the rust file will not compile because you are trying to use
    // a pointer thats already been freed.
    println!("{}", foo());

    // The next thing to talk about is the transfer of ownership
    // here we make a string literal and turn it into a growable string by using
    // the to_string method
    let var = "mouse".to_string();
    let house = var; // at this point we have transferred ownership of var to house
    println!("{}", house);
    // if we try to do something like
    // println!("{}", var);
    // we are going to get an error because var no longer has that value associated with it
    // However, this is ONLY for variables that could have variable length at compile time
    // so if we try to do
    let x = 5;
    let y = x;
    // we can still do
    println!("{} {}", x, y);

}

// This doesn't work 
// fn foo() -> &str {
//     return "ruh-roh";
// }
// because at this point the reference that would have been returned
// has already been freed which would result in a dangling pointer

// You can instead use a string literal, but if you need parts
// of the string to change at any point you should instead return
// a String which can grow in length
fn foo() -> &'static str {
    return "I am a fixed length string literal";
}

