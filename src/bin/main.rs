// working examples of the macros
//
// remember that the goal is to remake the if, else, and else if keywords using MBE macros

/*
 * to see impl of elif,
 * see the other two branches: main/iterator and main/recursive
 *
 *
 *
 * */

use cond_macros::my_if;

fn main() {
    let mut a = 1;
    my_if!(a==2 => {
      a = 111;
    }
    otherwise: {
        a = 123;
    });

    println!("{a}");
}
