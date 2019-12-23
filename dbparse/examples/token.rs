fn main() -> Result<(), std::io::Error> {
    use std::io::Write;
    print!("e-mail: ");
    std::io::stdout().flush()?;
    let mut input_email = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut input_email).expect("Non-utf8 string input!");

    print!("Type a pass: ");
    let pass = "TODO".to_string(); //passwd::read_password(); // TODO: how to read password without displaying it?
    println!("Your password is {}", pass);
    std::io::stdout().flush()?;

    let token = dbparse::get_auth_token(input_email.trim().as_ref(), pass.trim().as_ref());
    // TODO: trim will be a problem if the password contains whitespace
    println!("token: {}", token.unwrap());
    Ok(())

}
