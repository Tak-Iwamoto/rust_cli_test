use crate::app::CliApp;
mod app;
mod clap_app;
mod config;

fn main() {
    let app = CliApp::new();
    match app {
        Ok(a) => {
            println!("config: {:?}", a.matches);
            println!("config: {:?}", a.config().unwrap());
        }
        Err(_) => println!("error"),
    };
}
