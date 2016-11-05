// The `key` closure passed to `sort_by_key` can't borrow references from its argument.

fn main() {
    struct Student {
        first_name: String,
        last_name: String,
        gpa: i32  // you can have a GPA of up to two billion in this school
    }

    let mut students = vec![
        Student { first_name: "Joleen".to_string(), last_name: "Fritz".to_string(), gpa: 342 },
        Student { first_name: "Fritz".to_string(), last_name: "Brandt".to_string(), gpa: 4 },  // underachiever
        Student { first_name: "Erma".to_string(), last_name: "Brandt".to_string(), gpa: 519 },
    ];

    students.sort_by_key(|s| &s.last_name);  // error: can't infer lifetime
    //~^ ERROR: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
}
