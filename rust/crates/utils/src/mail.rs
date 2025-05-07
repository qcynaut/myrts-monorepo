/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use lettre::{
    message::header,
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        PoolConfig,
    },
    Message, SmtpTransport, Transport,
};

const CONFIRM_TEMPLATE: &'static str = include_str!("./mail/confirm_mail.html");
const RESET_TEMPLATE: &'static str = include_str!("./mail/reset_mail.html");

/// Mail.
/// The mailing utility.
#[derive(Clone)]
pub struct Mail {
    transport: SmtpTransport,
    from: String,
}

impl Mail {
    /// Create a new Mail.
    pub fn new(
        host: &str,
        username: &str,
        password: &str,
        from: &str,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = SmtpTransport::starttls_relay(host)?
            .credentials(Credentials::new(username.to_owned(), password.to_owned()))
            .authentication(vec![Mechanism::Plain])
            .pool_config(PoolConfig::new().max_size(20))
            .port(port)
            .build();
        Ok(Mail {
            transport,
            from: from.to_string(),
        })
    }

    /// Send an email.
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mail = Message::builder()
            .header(header::ContentType::TEXT_HTML)
            .from(self.from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_owned())?;
        self.transport.send(&mail)?;
        Ok(())
    }

    /// Send confirmation email.
    pub fn send_confirmation(
        &self,
        name: &str,
        to: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html = CONFIRM_TEMPLATE
            .replace("{{{url}}}", url)
            .replace("{{{name}}}", name);
        self.send(to, "MYRTS - Login confirmation", &html)
    }

    /// Send reset email.
    pub fn send_reset(
        &self,
        name: &str,
        to: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html = RESET_TEMPLATE
            .replace("{{{url}}}", url)
            .replace("{{{name}}}", name);
        self.send(to, "MYRTS - Reset password", &html)
    }
}
