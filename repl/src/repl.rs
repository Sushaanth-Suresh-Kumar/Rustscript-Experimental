use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{self, Write};

pub fn start() {
    let ascii_art = r#"
                              
            -+++*+.           
           %+  .:-%=          
        . .@+- :-* +#-        
      -#%: +@%: -.   =#+.     
    .#%:   =#%@+*+=:   -%*    
    %*  ..  .-=@         ##   
   *%  .*=     @   ..     @=  
   @+  :@@-.  *@. :#      +#  
   @+    :+@@#=%. -%+.    +#  
   *%      =@ .%  #-.*    %=  
    %*      *%#.-#:      *#   
    .%%:    +=:##.    *-%#    
      -#%%*%*+- .=%%#%@#:     
        .=*#*+==#@@%*=.       

    "#;

    println!("{}", ascii_art);
    println!("Welcome to the Monkey Programming Language!");
    // Try adding an ascii art of a Monkey Lang
    println!("Please feel free to try out the language...");

    let mut rl = DefaultEditor::new().expect("unable to load the default editor");
    // actual repl starts here
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("encountered problem when entering into history");
                println!("you entered: {:?}", line);
            }
            Err(ReadlineError::Interrupted) => {
                print!("\x1b[1A\x1b[2K\r>> ^C");
                io::stdout()
                    .flush()
                    .expect("failed to flush stdout when interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                print!("\x1b[1A\x1b[2K\r>> ^D");
                io::stdout()
                    .flush()
                    .expect("failed to flush stdout when eof");
                break;
            }
            Err(e) => {
                // Handle any other errors
                eprintln!("error reading input: {}", e);
                break;
            }
        }
    }
}
