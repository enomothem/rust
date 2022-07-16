// -- comments --
// comments 1
/* comments 2 */
/*
 *  comments 3
 *  comments 3
 *  comments 3
 */
/// comments 4
/// comments 4

fn main() {

    // var

    let a = 12;


    // -- print --

    println!("a is {}", a);
    println!("a is {}, a again is {}", a, a); 
    println!("a is {0}, a again is {1}", a, a); 
    println!("a is {0}, a again is {0}", a); 
    println!("a is {}, a again is {0}{{}}", a); 

    // -- func --

    another_function(1,1);
    
    // -- if --

   
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }

}

fn another_function(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

