struct File {
    descriptor: i32
}

fn new_file(d: i32) -> File {
    File { descriptor: d }
}

fn clone_from(this: &mut File, rhs: &File) {
    close(this.descriptor);
    this.descriptor = dup(rhs.descriptor);
}

fn open(filename: &str, mode: u32) -> i32 { 0 }
fn dup(fd: i32) -> i32 { fd + 1 }
fn close(fd: i32) {}

fn main() {
    let mut f = new_file(open("foo.txt", 0));
    clone_from(&mut f, &f);
    //~^ ERROR: cannot borrow `f` as immutable because it is also borrowed as mutable
}
