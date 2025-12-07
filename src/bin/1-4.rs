// Next we will cover structs
// This is the format in which you declare the definition
// #[derive(Debug)] is a trait that you can add to your struct
// so that it will automatically generate an implementation of Debug
// we will see later what this does
#[derive(Debug)]
struct House {
    walls: i32,
    _roof: i32,
    _doors: i32,
    _garage: i32,
    // if you want a field to be publicly available
    // you have to explicitly say so with the pub
    // keyword
    pub chairs: i32,
    _spiders: i32,
}
// next if you want to add functionality to the struct you have
// to implement it, you can do this with the impl keyword
impl House {
    // the new function is what you will use to create your own Structs in your code
    pub fn new(
        walls: i32,
        _roof: i32,
        _doors: i32,
        _garage: i32,
        chairs: i32,
        _spiders: i32,
    ) -> Self {
        House {
            walls,
            _roof,
            _doors,
            _garage,
            chairs,
            _spiders,
        }
    }
    // in our struct we can define basic getters and setters
    pub fn get_walls(&self) -> i32 {
        self.walls
    }
    // in our setter though we have to declare that the reference
    // to our struct is mutable so that we can actually change the
    // value of the walls
    pub fn set_walls(&mut self, new_walls: i32) {
        self.walls = new_walls;
    }
    // you probably noticed that the getter and setter I defined used
    // the pub keyword, this means that elsewhere in our code we can use
    // those functions. But if you want a private function you just need
    // to exclude pub
    fn _secret_santa() -> String {
        "Ho Ho Ho".to_string()
    }
} 

fn main() {
    // Here to make our own House from a struct we use the new function
    // we created. Since we are making a new House, we have to use :: with
    // 'House' to access the new function from the House definition
    // Since we plan to use the setter, we will also have to make our house
    // mutable
    let mut spider_house = House::new(4, 1, 2, 1, 10, 20);
    // here we can access our getter function
    println!("{}", spider_house.get_walls());
    // we can also use our setter function
    spider_house.set_walls(8);
    println!("{}", spider_house.get_walls());
    // We can also access the the public field of chairs
    println!("{}", spider_house.chairs);
    // Because we derived the Debug trait, we can do {:?} in our println
    // to see all the values of our spider_house without having to add any
    // implementation of our own.
    println!("{:?}", spider_house);
}