// Copy of samples/examples/languages.rs, but with an extra println! added.
// The book claims it'll fail with a particular message.

fn main() {
    // Get our command-line arguments as a vector of Strings.
    let languages : Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
    //~^ NOTE: `languages` moved here because it has type `collections::vec::Vec<collections::string::String>`, which is non-copyable
        println!("{}: {}", l,
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }
    println!("{} languages classified", languages.len());
    //~^ ERROR: use of moved value: `languages`
}
