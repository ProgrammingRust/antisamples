// Macros won't sweep a syntax error under the rug and match a different rule.
// If Rust starts trying to match `$msg:expr` but it doesn't match, it's fatal.

macro_rules! complain {
    ($msg:expr) => {
        println!("Complaint filed: {}", $msg);
    };
    (user : $userid:tt , $msg:expr) => {
        println!("Complaint from user {}: {}", $userid, $msg);
    };
}

fn main() {
    complain!(user: "jimb", "the AI lab's chatbots keep picking on me");
    //~^ ERROR: expected type, found `"jimb"`
}
