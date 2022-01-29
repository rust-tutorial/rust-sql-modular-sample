pub const GET: &str = "GET";
pub const POST: &str = "POST";
pub const PUT: &str = "PUT";
pub const PATCH: &str = "PATCH";
pub const DELETE: &str = "DELETE";
pub const OPTIONS: &str = "OPTIONS";

pub fn is_method(s: String) -> bool {
    match s {
        m if m.contains(GET) => true,
        m if m.contains(POST) => true,
        m if m.contains(PUT) => true,
        m if m.contains(PATCH) => true,
        m if m.contains(DELETE) => true,
        m if m.contains(OPTIONS) => true,
        _ => false,
    }
}
