use lettre::{self, transport::smtp::authentication::Credentials, SmtpTransport, Transport};

mod settings;

fn main() {
    let config_file: String = std::env::args().skip(1).take(1).collect();
    let config = settings::Settings::new(config_file.as_str()).unwrap();
    let email = lettre::Message::builder()
        .from(config.username().parse().unwrap())
        .to(config.destination().parse().unwrap())
        .subject(config.subject())
        .body(config.message().to_owned())
        .unwrap();

        let creds = Credentials::new(config.username().to_owned(), config.password().to_owned());

        let mailer = SmtpTransport::relay(&config.mail_server())
            .unwrap()
            .port(config.smtp_port())
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent"),
            Err(e) => println!("Could not send email: {}", e),
        }
}
