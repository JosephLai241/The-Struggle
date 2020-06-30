use ansi_term::*;

pub fn main_title() {
    let main = r#"
 __             
/\ \__     👎   
\ \ ,_\   ____  
 \ \ \/  /',__\ 
  \ \ \_/\__, `\
   \ \__\/\____/
    \/__/\/___/
"#;
    println!("{}",Colour::Red.bold().paint(main))
}
