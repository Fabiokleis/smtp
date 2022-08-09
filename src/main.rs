use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let email = Message::builder()
        .from("urameshi <fabiokleis.shell@gmail.com>".parse().unwrap())
        .reply_to("NoBody <nobody@domain.tld>".parse().unwrap())
        .to("fabiokleis <fabiohkrc@gmail.com>".parse().unwrap())
        .subject("Test")
        .body(String::from("Ignore, this is a test mail!"))
        .unwrap();

    let user = env::var("EMAIL").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let creds = Credentials::new(user, password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
