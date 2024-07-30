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

    // actual repl starts here
    loop {
        let mut input = String::new();

        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                // End of file (Ctrl+Z or Ctrl+D)
                println!("^Z\nEnd of input detected. Exiting.");
                break;
            }
            Ok(_) => {
                // Successful read, process the input
                println!("You entered: {}", input.trim());
            }
            Err(e) => {
                // Handle any other errors
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
