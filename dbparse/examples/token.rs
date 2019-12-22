fn main() {
    use std::io::Write;
    print!("e-mail: ");
    std::io::stdout().flush();
    let mut input_email = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut input_email).expect("Non-utf8 string input!");
    print!("passwort: ");
    std::io::stdout().flush();
    let mut input_pass = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut input_pass).expect("Non-utf8 string input!");

    let token = dbparse::get_auth_token(input_email.as_ref(), input_pass.as_ref());
    println!("token: {}", token.unwrap());

}
