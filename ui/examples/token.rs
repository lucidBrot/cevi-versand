fn main() {
    use ui::UserInteractor;
    let user_interface = ui::CliUi {};
    let token_res = user_interface.interactively_get_auth_token();
    match token_res {
        Err(_e) => (), // Errors have already been shown to the user
        Ok(data) => {
            println!("Overwriting config file...");
            dbparse::generate_template_config_file(
                data.login_email.as_ref(),
                data.user_token.as_ref(),
                "th1s1sY0ur53rvic370k3n",
            );
        },
    }
}
