// The following error message appears in the book:
// error-pattern: function is never used: `is_square`

#![deny(dead_code)]

mod plant_structure {
    pub mod roots {
        #[derive(Debug)]
        pub struct Root;

        pub struct Shape;

        static ROUND: Shape = Shape;

        impl Root {
            pub fn cross_section_shape(&self) -> &Shape { &ROUND }
        }

        impl Shape {
            pub fn is_square(&self) -> bool { false }
        }

        pub fn is_square(root: &Root) -> bool {
            root.cross_section_shape().is_square()
        }
    }

    pub fn make_root() -> roots::Root { roots::Root }
}

fn main() {
    use plant_structure::make_root;
    println!("{:?}", make_root().cross_section_shape().is_square());
}
