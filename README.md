# Hex to from

## Usage

```rust 
cargo run
```
## Change content of file hexx.txt or change of file
```rust 
 │
  18   │ fn main(){
  19   │
  20   │     if let Ok(lines) =  read_lines("hexx.txt"){
  21   │         for line in lines{
  22   │             if let Ok(line_content) = line{
  23   │                 let clean = line_content.replace(" ", "");
  24   │                 let convert_to_text = from_hex_to_text(clean.as_bytes());
  25   │                 println!("{}", convert_to_text);
  26   │             } else {
  27   │                 print!("Error");
  28   │             }
  29   │         }
  30   │     }
  31   │ }
```
