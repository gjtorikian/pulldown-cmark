// This file is auto-generated by the build script
// Please, do not modify it manually

extern crate pulldown_cmark;

include!("normalize_html.rs.inc");


    #[test]
    fn gfm_strikethrough_test_1() {
        let original = r##"~~Hi~~ Hello, world!
"##;
        let expected = r##"<p><del>Hi</del> Hello, world!</p>
"##;

        use pulldown_cmark::{Parser, html, Options};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TASKLISTS);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p);

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn gfm_strikethrough_test_2() {
        let original = r##"This ~~has a

new paragraph~~.
"##;
        let expected = r##"<p>This ~~has a</p>
<p>new paragraph~~.</p>
"##;

        use pulldown_cmark::{Parser, html, Options};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_FOOTNOTES);
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        opts.insert(Options::ENABLE_TASKLISTS);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p);

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }