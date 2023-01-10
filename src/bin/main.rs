// working examples of the macros
//
// remember that the goal is to remake the if, else, and else if keywords using MBE macros

use cond_macros::*;

fn main() {
    let mut a = 3;

    my_if!(elif a==1 => {
      a = 111;
    }
    elif a==2 => {
        a = 222;
    }
    elif a==3 => {
        a = 333;
    }
    elif a==3 => {
        a = 333;
    }
    otherwise => {
        a = 123;
    });

    println!("{a}");
}
