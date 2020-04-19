#![feature(vec_remove_item)]

pub trait BitCheck {
    fn one_bit_flipped(&self, other: usize) -> bool;
}

impl BitCheck for usize {
    fn one_bit_flipped(&self, other: usize) -> bool {
        (*self ^ other).is_power_of_two()
    }
}

pub struct BitChecker {
    collection: Vec<usize>,
    perms: Vec<Vec<usize>>,
}

impl BitChecker {
    pub fn new() -> Self {
        BitChecker {
            collection: Vec::new(),
            perms: Vec::new(),
        }
    }

    pub fn get_all_perms(&mut self, combinations: usize) -> Option<&Vec<Vec<usize>>> {
        self.perms.clear();
        self.collection = (0..combinations + 1).collect();

        let choices: Vec<usize> = (1..combinations + 1).collect();
        self.get_perms(0, &choices, Vec::new());

        if self.perms.len() == 0 {
            None
        } else {
            Some(&self.perms)
        }
    }

    pub fn get_perms(&mut self, start: usize, choices: &[usize], route: Vec<usize>) {
        let mut this_route: Vec<usize> = route.clone();
        this_route.push(start);
        for choice in choices {
            if start.one_bit_flipped(*choice) {
                let mut next_choices: Vec<usize> = choices.clone().into();
                next_choices.remove_item(choice);
                self.get_perms(*choice, &next_choices, this_route.clone());
            }
        }
        if choices.len() == 0 {
            self.perms.push(this_route);
        }
    }
}

fn main() {
    let mut bc = BitChecker::new();
    match bc.get_all_perms(15) {
        None => println!("No result"),
        Some(res) => {
            for perm in res {
                for i in 1..perm.len() {
                    let card = match perm[i] ^ perm[i - 1] {
                        1 => 1,
                        2 => 2,
                        4 => 3,
                        8 => 4,
                        _ => unreachable!(""),
                    };
                    print!("{} ", card);
                }
                println!("");
                for el in perm {
                    print!("{:04b} ", el);
                    // print!("{} ", el);
                }
                println!("");
            }
        }
    }
}
