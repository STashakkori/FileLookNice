// -$t@$h     QVLx Labs

use std::io::{self, Read};

fn main() -> io::Result<()> {
   let mut buffer = String::new();
   let stdin = match io::stdin().read_to_string(&mut buffer) {
     Ok(o) => o,
    Err(e) => { println!("Stdin error: {}", e); return Err(e); }
   };
   let split_nl = &buffer.split("\n").collect::<Vec<&str>>();
   
   // Loop through files passed in through stdin
   for i in 0..split_nl.len() - 1 { // Exclude very last newline, there is nothing after
		 if split_nl[i].contains(",") { // See if output contains commas
			 let split_cma = &split_nl[i].split(",").collect::<Vec<&str>>(); // Tokenize on commas
			 if split_cma.len() > 1 {
         let first = split_cma[0];
         let first_cln = &first.split(":").collect::<Vec<&str>>(); 
         let file_name = first_cln[0];
         let profile = first_cln[1].trim_left();
				 if !profile.contains("directory") { 
					 println!("\x1b[38;5;138m{}\x1b[0m", file_name);
         }
         if split_cma.len() > 1 {
				   println!(" \x1b[38;5;101m╠ \x1b[38;5;115m{}\x1b[0m", profile);
					 for j in 1..split_cma.len() {
					   if j == 1 {
               println!(" \x1b[38;5;101m╠ \x1b[38;5;141m{}\x1b[0m", split_cma[1].trim_left());
             } 
             else if j == (split_cma.len() - 1) {
							 println!(" \x1b[38;5;101m╚ \x1b[38;5;110m{}\x1b[0m", split_cma[j].trim_left());
						 }
						 else {
							 println!(" \x1b[38;5;101m╠ \x1b[38;5;110m{}\x1b[0m", split_cma[j].trim_left());
						 }
					 }
         }
         else {
				   println!(" \x1b[38;5;101m╚ \x1b[38;5;115m{}\x1b[0m", profile);
         }
			 }
		 }
     // Handle directories
		 else {
			 let head = &split_nl[i].split(":").collect::<Vec<&str>>(); 
			 let file_name = &head[0][..];
			 let profile = &head[1];
			 println!("\x1b[38;5;138m{}\x1b[0m", file_name);
       if profile.contains("directory"){
			   println!(" \x1b[38;5;101m╚ \x1b[38;5;115m{}\x1b[0m", profile.trim_left());
       }
       else {
			   println!(" \x1b[38;5;101m╚ \x1b[38;5;115m{}\x1b[0m", profile.trim_left());
		   }
     }
   }
   Ok(())
}
