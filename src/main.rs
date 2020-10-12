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
                    link rel="stylesheet" type="text/css" href=(format!("markdown-css-themes/{}.css", s)) {}
                } else {
                    link rel="stylesheet" type="text/css" href="markdown-css-themes/foghorn.css" {}
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
                            (@arg css: --css +takes_value "Name of your style. Available styles: avenir-white, foghorn")
                            (@arg file: -f +takes_value "The name of the html file to store")
                            (@arg debug: -d "Display additional debug information")
    )
    .get_matches();

    let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read markdown file!");
    let mut res = String::new();

    // Lazy - will just return one thing at a time
    let parser = Parser::new(&infile);
    push_html(&mut res, parser.into_iter());
 
    
    res = wrap_in_html(&res, clap.value_of("css"));


    if clap.is_present("debug") {
        println!("{:?}", res);
    }

    let mut output_file = "";
    match clap.value_of("file") {
        Some(s) => {
            output_file = s;
        },
        Error => { 
            output_file = "output.html";
        }
    }


    write_to_file(output_file, &res);


   
    // open a file in a browser window
    Command::new("open")
        .arg(output_file)
        .output();

}
