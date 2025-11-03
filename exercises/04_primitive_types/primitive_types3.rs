fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a = load_array();

    fn load_array() -> [i32;100] {
        let mut a = [0; 100]; //array w/ 100 elm init to 0
        
        for number in 0..100{
            a[number] = number as i32; 
        }

        a

    }

    //lol I could have jsut solved it with 
    //let a = [0;100]; inits' that array with 100 elements in it. 
    

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }


    
}
