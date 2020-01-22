use actix_files as fs;
use actix_web::{App, HttpServer};

extern crate clap;
use clap::{Arg};
use clap::App as ClapApp;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("My Super Program")
        .version("1.0")
        .author("Daniil N. <daniil.naumetc@gmail.com>")
        .about("Creates a file server. Do not use for production")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Sets a custom port")
            .takes_value(true)
            .default_value("1337"))
        .arg(Arg::with_name("directory")
            .short("d")
            .long("directory")
            .value_name("DIRECTORY")
            .help("Sets a custom directory")
            .takes_value(true)
            .default_value("."))
        .get_matches();
    
    let port = matches.value_of("port").unwrap();
    let directory = String::from(matches.value_of("directory").unwrap());


    HttpServer::new(move || {
        App::new().service(fs::Files::new("/", &directory).show_files_listing()) // /static
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}