fn main() {
    // Now we are going to go over Options.
    // Essentially rust doesn't want you returning a null value
    // when what you are looking for is missing
    let example = vec![1, 2, 3, 5, 8, 13, 21, 34, 55];
    
    // One way to handle Options is to match keyword to
    // handle both cases cleanly
    match find_item(&example, 13) {
        Some(index) => println!("{} was the index", index),
        None => println!("not a fib number"),
    }

    // Another way is the unwrap function. However, you should
    // refrain from using this normally since if None is returned
    // your program will panic and crash. (I am guilty of using this
    // too much when I started writing in Rust because its too easy
    // to use for happy paths)
    let idx = find_item(&example, 21).unwrap();
    println!("{}", idx);
    // Example that will cause an error
    // let idx = find_item(&example, 4).unwrap();
    
    // Although unwrap is frowned upon there are other similar functions
    // that handle the None case, like unwrap_or
    let idx2 = find_item(&example, 22).unwrap_or(0);
    // 22 wasn't in the number vector so 0 will be printed here
    // the one caveat with unwrap_or though is that whatever default value
    // you supply influences the type of the variable. So you just have to
    // make sure that your default value is the same type as whatever is in
    // the Option
    println!("{}", idx2);

    // There are a plethora of other ways to handle Options and if you want
    // to read more you can learn more in the rust doc https://doc.rust-lang.org/std/option/
}

fn find_item(vec: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in vec.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}
