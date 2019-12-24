fn main() -> Result<(), std::io::Error> {
    use std::io::Write;
    print!("e-mail: ");
    std::io::stdout().flush()?;
    let mut input_email = String::new();
    let _bytes_read = std::io::stdin()
        .read_line(&mut input_email)
        .expect("Non-utf8 string input!");

    print!("Type a pass: ");
    std::io::stdout().flush()?;
    let mut pass = String::new();
    let _bytes_read = std::io::stdin()
        .read_line(&mut pass)
        .expect("Non-utf8 string input for password!");
    // TODO: how to read password without displaying it?
    std::io::stdout().flush()?;

    let auth_token = dbparse::get_auth_token(input_email.trim().as_ref(), pass.trim().as_ref());
    match auth_token {
        Err(e) => {
            println!("Problem fetching token: {:?}", e);
            match e.kind() {
                std::io::ErrorKind::Other => println!("Perhaps your internet connection is down?"),
                std::io::ErrorKind::InvalidData => println!(
                    "Perhaps the credentials were wrong, perhaps the database is down, ..."
                ),
                _ => (),
            }
            Ok(())
        },
        Ok(token) => {
            // TODO: trim will be a problem if the password contains whitespace
            println!("Auth Token: {:?}", token);
            Ok(())
        },
    }
}
