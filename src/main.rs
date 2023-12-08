use std::{fs::File,path::Path,io::{self, BufRead}};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn from_hex_to_text(buffer: &[u8]) -> String{ 
    let hex_string: String = buffer.iter().cloned().map(char::from).collect();
    let buffer = hex::decode(hex_string).expect("Failed to convert hex to string");
    let string = String::from_utf8_lossy(&buffer);
    string.into_owned()
}


fn main(){
    if let Ok(lines) =  read_lines("hexx.txt"){
        for line in lines{
            if let Ok(line_content) = line{
                let clean = line_content.replace(" ", "");
                let convert_to_text = from_hex_to_text(clean.as_bytes());
                println!("{}", convert_to_text);
            } else {
                print!("Error");
            }
        }
    }
}


#[cfg(test)]
mod test{
    use crate::from_hex_to_text;

    use super::read_lines;
    
    #[test]
    fn test_read_file(){
        if let Ok(lines) = read_lines("hexx.txt"){
            for line in lines{
                println!("{:?}", line);
                assert_eq!(
                    line.unwrap(),
                    "48 6f 6c 6c 6f 20 77 6f 72 6c 64"
                );
            }
        } else {
            print!("Error")
        }
    }

    #[test]
    fn test_from_hex_to_text(){
        if let Ok(lines) =  read_lines("hexx.txt"){
            for line in lines{
                if let Ok(line_content) = line{
                    let clean = line_content.replace(" ", "");
                    let convert_to_text = from_hex_to_text(clean.as_bytes());
                    println!("{}", convert_to_text);
                } else {
                    print!("Error");
                }
            }
        }
    }
}
