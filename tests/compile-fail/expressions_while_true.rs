// `while true` confuses Rust's path-sensitive analyses

// error-pattern: mismatched types

struct Process;
impl Process {
    fn exit_code(&self) -> i32 { 0 }
    fn wait(&mut self) -> bool { true }
}

// The book claims that rustc flags this function with a particular error,
// which is actually untrue:
fn wait_for_process(process: &mut Process) -> i32 {
    while true {
        if process.wait() {
            return process.exit_code();
        }
    }
}  // error: not all control paths return a value

fn main() {}
