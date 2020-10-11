use std::process::Command;
use clap::{clap_app, crate_version};
use pulldown_cmark::{html::push_html, Event, Parser};
use maud::html;

// Wrap an HTML string with a valid HTML elemnets such as head and body
// Optionally include a link to CSS file
fn wrap_in_html(s: &str, css: Option<&str>) -> String {
    let res = html!{
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                @if let Some(s) = css {
                    link rel="stylesheet" type="text/css" href=(s) {}
                }
                body {
                    (maud::PreEscaped(s))
                }
            }
        }
    };

    res.into_string()
}

fn write_to_file(file: &str, contents: &str) -> std::io::Result<()> {
    
    std::fs::write(file, contents)?;

    Ok(())
}

fn main() {
    
    let clap = clap_app!( md2html => 
                            (version:crate_version!())
                            (author: "Andrey I. Esaulov")
                            (about: "Convert markdown into the best looking HTML you've ever seen.")
                            (@arg input: +required "Input markdown file.")
                            (@arg wrap: -w "Wrap in html")
                            (@arg css: --css +takes_value "Link to css")
    )
    .get_matches();

    let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read markdown file!");
    let mut res = String::new();

    // Lazy - will just return one thing at a time
    let parser = Parser::new(&infile);
    push_html(&mut res, parser.into_iter());
 
    if clap.is_present("wrap") {
        res = wrap_in_html(&res, clap.value_of("css"));
    }

    println!("{:?}", res);


    write_to_file("output.html", &res);
    // open a file in a browser window
    Command::new("open")
        .arg("output.html")
        .output();

}
