// working examples of the macros
//
// remember that the goal is to remake the if, else, and else if keywords using MBE macros

use cond_macros::my_if;

fn main (){
    // some syntax ideas
    let a = 1;
    let b = 1;
    my_if!(a==b => {
        /* some code here*/
    });

    todo!()
}
