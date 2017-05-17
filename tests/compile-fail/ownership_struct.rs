// By default, struct and enum types are not copyable.

// error-pattern: use of moved value: `l.number`

fn main() {
    struct Label { number: u32 }

    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}
