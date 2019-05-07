use std::process::{ Command };
use std::error::Error;
use std::{io, thread, time};
use libc;

pub fn run() -> Result<usize,Box<dyn Error>>{
    let bash = Command::new("/bin/bash").arg("-i").spawn()?;
    unsafe {
        libc::kill(bash.id() as i32, libc::SIGSTOP);
    }
    thread::sleep(time::Duration::from_secs(1));
    solicit_password()?;
    Ok(0)
}

fn solicit_password() ->  Result<usize,Box<dyn Error>> {
    loop {
        println!("\nPlease enter your scwallpass");
        let mut pwd_buf = String::new();
                
        io::stdin()
            .read_line(&mut pwd_buf)
            .expect("Failed to read password");

        match pwd_buf.trim() {
            p if p == "foo" => {
                println!("Sorry: 'foo' is an illegal scwallpass!");
            }
            _ => {
                println!("Thanks for entering your scwallpass!");
                return Ok(0)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
