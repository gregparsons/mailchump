use mail_send::mail_builder::MessageBuilder;
use mail_send::Transport;
use dotenvy::dotenv;
use std::env;

fn main() {

    dotenv().ok();

    let server = env::var("SMTP_SERVER").unwrap();

    // Build a simple multipart message
    let message = MessageBuilder::new()
        .from(("John Doe", "john@example.com"))
        .to(vec![
            ("Jane Doe", "jane@example.com"),
            ("James Smith", "james@test.com"),
        ])
        .subject("Hi!")
        .html_body("<h1>Hello, world!</h1>")
        .text_body("Hello world!");

    // Connect to an SMTP relay server over TLS and
    // authenticate using the provided credentials.
    Transport::new(server)
        .credentials("john", "p4ssw0rd")
        .connect_tls()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();


}
