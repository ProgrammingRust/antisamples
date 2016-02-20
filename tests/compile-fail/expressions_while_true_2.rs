// See expressions_while_true.rs for explanation.

struct Process;
impl Process {
    fn exit_code(&self) -> i32 { 0 }
    fn wait(&mut self) -> bool { true }
}

fn wait_for_process_2(process: &mut Process) -> i32 {
//~^ ERROR: not all control paths return a value
    while true {
        if process.wait() {
            return process.exit_code();
        }
    };
}

fn main() {}
