use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    mail_server: String,
    smtp_port: u16,
    username: String,
    password: String,
    subject: String,
    message: String,
    destination: String,
}

impl Settings {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name(file))
            .build()?;

        conf.try_deserialize()
    }

    pub fn mail_server(&self) -> &str {
        &self.mail_server
    }

    pub fn smtp_port(&self) -> u16 {
        self.smtp_port
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let settings = Settings::new("resources/test/config.yaml").unwrap();
        assert_eq!(settings.mail_server, "mail.example.com");
        assert_eq!(settings.username, "testuser".to_owned());
        assert_eq!(settings.password, "testpassword".to_owned());
    }
}
