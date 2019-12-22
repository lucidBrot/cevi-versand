fn main() {
    print!("e-mail: ");
    let mut input_email = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut input).expect("Non-utf8 string input!");
    print!("passwort: ");
    let mut input_pass = String::new();
    let _bytes_read = std::io::stdin().read_line(&mut input).expect("Non-utf8 string input!");

    let token = dbparse::get_auth_token(input_email, input_pass);
    println!("token: {}", token);

}
