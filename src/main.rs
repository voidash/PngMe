
// mod args;
mod chunk;
mod chunk_types;
mod command_args;
mod png;
mod test;
mod utils;


use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author="Ashish Thapa")]
#[clap(about, version, author)]
struct Args {

    /// mode: encode or decode
    #[clap(long)]
    mode: String,

    /// png file path
    #[clap(short,long)]
    file: String, 

    /// 4 character String, 3rd character must be capital
    #[clap(short,long)]
    chunk: String ,


    /// message to hide
    #[clap(short,long,)]
    message: Option<String>, 
}

pub type Error = Box<dyn std::error::Error>; // trait object is prefixed with dyn
pub type Result<T> = std::result::Result<T,Error> ;

fn main() {
    let args = Args::parse();

    

    match args.mode.as_str() {
        "decode" => {
            utils::decode(args.file.as_str(), args.chunk.as_str());
        },
        "encode" => {
            // message is required
            let message = args.message.expect("message needs to be passed for encode mode");
            utils::encode(args.file.as_str(),message.as_str(),args.chunk.as_str());

            println!("{}-{}",  args.chunk, message);
            println!("Message encoded Successfully.");
        }
        _ => {
            panic!("Mode can only be encode or decode");
        }
    }
    // let args = command_args::get_parameters();
    // if args.len() == 4 {
    //     //encode
    //     utils::encode(&args[1], &args[3], &args[2]);
    // }

    // if args.len() == 3 {
    //     //decode
    //     utils::decode(&args[1], &args[2]);
    // }

}
