// Copy of samples/examples/languages.rs, but with an extra println! added.
// The book claims it'll fail with a particular message.

// error-pattern: use of moved value: `languages`

fn main() {
    // Get our command-line arguments as a vector of Strings.
    let languages : Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }
    println!("{} languages classified", languages.len());
}
