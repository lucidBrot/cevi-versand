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

    let token = dbparse::get_auth_token(input_email.trim().as_ref(), pass.trim().as_ref()).unwrap();
    // TODO: trim will be a problem if the password contains whitespace
    println!("token: {}", &token);
    
    let yaml : serde_yaml::Value = serde_yaml::from_str(token.as_ref()).unwrap();
    let auth_token : &serde_yaml::Value = yaml.get("people").unwrap().get(0).unwrap().get("authentication_token").unwrap();
                                   println!("Auth Token: {:?}", auth_token);

    Ok(())
}
