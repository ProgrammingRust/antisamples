// `while true` confuses Rust's path-sensitive analyses

// error-pattern: mismatched types

struct Process;
impl Process {
    fn exit_code(&self) -> i32 { 0 }
    fn wait(&mut self) -> bool { true }
}

// The book claims that rustc flags this function with a particular error,
// which is actually a little white lie:
fn wait_for_process(process: &mut Process) -> i32 {
    while true {
        if process.wait() {
            return process.exit_code();
        }
    }
}  // error: not all control paths return a value

// But the same program with one semicolon added *does* give that error message
// (see expressions_while_true_2.rs)

fn main() {}
