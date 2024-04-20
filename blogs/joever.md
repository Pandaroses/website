---
title = "why making a SSG in a build script is a dumb idea"
description = "this mistake will be rectified "
date = "20.4.24"
---

a few weeks ago i decided to make a personal website of my own, i've tried to do it multiple times before, but each time just got bored & gave up.
unfortunately, the silly cat brain inside of me decided that writing the website in rust would be awesome & cool(i still think that)

## the good
- in my opinion, the website looks pretty
- decently fast
## the bad 
- code strewn over the place
- weird url conventions
- will be completely rewritten

## the ugly
- the code looks like this
```rust 
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

  
```

in conclusion, this site will be rewritten in about 10 business years. thank you for your eyes.



