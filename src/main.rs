use mail_send::mail_builder::MessageBuilder;
use mail_send::Transport;
use dotenvy::dotenv;
use std::env;


fn main() {

    dotenv().ok();


    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    let _ = rt.block_on(
        send_email("subject here",
                   "<h1>Hello, world!</h1>",
                   "Hello world!",
                   "from_me@example.com",
                   "From Me",
                   vec![
                       ("Jane Doe".to_string(), "jane@example.com".to_string()),
                       ("James Smith".to_string(), "james@test.com".to_string()),
                   ]
        )
    );
}

async fn send_email(
    subject:&str, html_body:&str, text_body:&str,
    from_email:&str, from_name:&str,
    recipient_list:Vec<(String, String)>
){

    let server = env::var("SMTP_SERVER").unwrap();
    // let username = env::var("SMTP_USER").unwrap();
    // let password = env::var("SMTP_PW").unwrap();

    let message = MessageBuilder::new()
        .from((from_name, from_email))
        .to(recipient_list)
        .subject(subject)
        .html_body(html_body)
        .text_body(text_body);

    // Connect to an SMTP relay server over TLS and
    // authenticate using the provided credentials.
    Transport::new(server)
        .port(2525)
        // .credentials(username, password)
        // .connect_tls()
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();

}