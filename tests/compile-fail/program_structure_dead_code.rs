// The following error message appears in the book:
// error-pattern: function is never used: `is_square`

#![deny(dead_code)]

mod plant_structure {
    pub mod roots {
        #[derive(Debug)]
        pub struct Root;

        pub fn is_square(root: &Root) -> bool {
            let _ = root;
            false
                /* **** **** **** **** **** **** **** ****
                   (make function body long, so rustc won't
                   include all of it in the error message)
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** ****
                   **** **** **** **** **** **** **** **** */
        }
    }

    pub fn make_root() -> roots::Root { roots::Root }
}

fn main() {
    use plant_structure::make_root;
    println!("{:?}", make_root());
}

