// Names in macros can't refer to variables in the caller's scope.
// (Even if you want them to.)
//
// error-pattern: cannot find value `server_socket` in this scope
// error-pattern: cannot find value `req` in this scope

struct ServerSocket;
impl ServerSocket {
    fn session(&self) -> i32 { 0 }
}
type ServerRequest = Box<i32>;

macro_rules! setup_req {
    () => {
        let req = ServerRequest::new(server_socket.session());
    }
}

fn handle_http_request(server_socket: &ServerSocket) {
    setup_req!();  // declares `req`, uses `server_socket`
    drop(req); // code that uses `req`
}
