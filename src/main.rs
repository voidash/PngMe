
// mod args;
mod chunk;
mod chunk_types;
mod command_args;
mod png;
mod test;


pub type Error = Box<dyn std::error::Error>; // trait object is prefixed with dyn
pub type Result<T> = std::result::Result<T,Error> ;

fn main() {
    todo!()
}
