// working examples of the macros
//
// remember that the goal is to remake the if, else, and else if keywords using MBE macros

use cond_macros::my_if;

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

    // i want this expansion to look like
    let mut a = 1;
    {
        match a == 1 {
            true => {
                a = 111;
            }
            false => match a == 2 {
                true => {
                    a = 222;
                }
                false => match a == 3 {
                    true => {
                        a = 333;
                    }
                    false => a = 123,
                },
            },
        }
    }

    println!("{a}");
}
