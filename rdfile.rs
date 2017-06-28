use std::io::buffered::BufferedReader;
use std::io::File;

fn main()
{
    match File::open(&Path::new("message.txt")) {
        Some(file) => {
            let reader = BufferedReader::new(file);
            //reading from file
        }
        None =>{
            println("Opening message.txt failed!");
        }
    }
}
