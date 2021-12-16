
// mod args;
mod chunk;
mod chunk_types;
mod command_args;
mod png;
mod test;
mod utils;


pub type Error = Box<dyn std::error::Error>; // trait object is prefixed with dyn
pub type Result<T> = std::result::Result<T,Error> ;

fn main() {
    let args = command_args::get_parameters();

    if args.len() == 4 {
        //encode
        utils::encode(&args[1], &args[3], &args[2]);
    }

    if args.len() == 3 {
        //decode
        utils::decode(&args[1], &args[2]);
    }

}
