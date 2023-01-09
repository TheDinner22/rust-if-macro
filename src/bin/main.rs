// working examples of the macros
//
// remember that the goal is to remake the if, else, and else if keywords using MBE macros

use cond_macros::*;

fn main() {
    my_if!(a==1 => {
      a = 111;
    }
    elif a==2 => {
        a = 222;
    }
    elif a==3 => {
        a = 333;
    }
    otherwise => {
        a = 123;
    });

    // i would like this macro to expand to
    {
        const CHAIN_LEN: usize = (2) + 2; // todo make dyn

        struct IfChain {
            len: usize,
            completed: usize,
        }

        impl Iterator for IfChain {
            type Item = bool;

            fn next(&mut self) -> Option<Self::Item> {
                if self.completed == self.len {
                    None
                } // todo change if to match later

                // todo idk what i am doing and am going to try another approach
            }
        }
    }

    let a = 1;
    println!("{a}");
}
