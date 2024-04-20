use askama::Template;
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};
use comrak::{
    markdown_to_html_with_plugins, Options, Plugins, PluginsBuilder, RenderPlugins,
    RenderPluginsBuilder,
};
use serde::Deserialize;
use std::fs;
use std::process::Command;
use syntect::highlighting::ThemeSet;
const TAILWIND_VERSION: &str = "3.4.1";

#[derive(Deserialize, Debug, Clone)]
struct Header {
    title: String,
    description: String,
    date: String,
}

#[derive(Template)]
#[template(path = "blog.html")]
struct Blogs {
    blogs: Vec<Blog>,
}

struct Blog {
    header: Header,
    id: String,
    colour: String,
}

#[derive(Template)]
#[template(path = "awesome.html", escape = "none")]
struct Awesome {
    title: String,
    body: String,
}

fn main() {
    // println!("cargo::rerun-if-changed=blogs/*");
    // println!("cargo::rerun-if-changed=projects/*");
    Command::new(if cfg!(windows) { "cmd" } else { "sh" })
        .args([
            if cfg!(windows) { "/C" } else { "-c" },
            format!(
                "npx -- tailwindcss@{} -i templates/main.css -o static/main.css --minify",
                TAILWIND_VERSION
            )
            .as_str(),
        ])
        .output()
        .expect("failed to run tailwind, is npm installed??????");
    let mut blogs = Blogs { blogs: vec![] };
    let mut colours = vec![
        "rosewater",
        "flamingo",
        "pink",
        "mauve",
        "red",
        "maroon",
        "peach",
        "yellow",
        "green",
        "teal",
        "sky",
        "sapphire",
        "blue",
        "lavender",
    ]
    .into_iter()
    .cycle();
    for meow in fs::read_dir("blogs").unwrap() {
        let meow = meow.unwrap();
        let content = fs::read_to_string(meow.path()).unwrap();
        let iter = &mut content.splitn(3, "---\n");
        assert!(iter.next().unwrap().is_empty());
        let header: Header = toml::from_str(iter.next().unwrap()).unwrap();
        let mut id: String = meow
            .file_name()
            .to_string_lossy()
            .to_string()
            .split_once(".")
            .unwrap()
            .0
            .to_string();
        id.push_str(".html");
        blogs.blogs.push(Blog {
            header: header.clone(),
            id: id.clone(),
            colour: colours.next().unwrap().to_string(),
        });
        let mut options = Options::default();
        options.extension.math_code = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.math_dollars = true;
        let mut plugins = Plugins::default();
        let meow = SyntectAdapterBuilder::new()
            .theme_set(ThemeSet::load_from_folder("src").unwrap())
            .theme("mocha")
            .build();
        plugins.render.codefence_syntax_highlighter = Some(&meow);
        let awesome = Awesome {
            title: header.title,
            body: markdown_to_html_with_plugins(iter.next().unwrap(), &options, &plugins),
        };
        fs::write(format!("blog/{}", id), awesome.render().unwrap()).unwrap();
        fs::write("templates/truesay.html", blogs.render().unwrap()).unwrap();
    }
}
