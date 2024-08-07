Minimal reproducible example for a parsing issue encountered with the [xrust](https://github.com/ballsteve/xrust) crate.

## Issue

Calling `xrust::parser::xml::parse` with `Rc<xrust::trees::smite::Node>` as the first argument on [the XML file][XML file] in this repo returns an `Err` with the following message:

```
Unrecoverable parser error while parsing XML "[...]"
```

The [XML file] is valid according to the various online XML validators, such as:

-   https://www.xmlvalidation.com/
-   https://www.w3schools.com/xml/xml_validator.asp
-   https://jsonformatter.org/xml-validator

It is also successfully parsed by other XML parsers I used before in my project, specifically:

-   [roxmltree](https://github.com/RazrFalcon/roxmltree)
-   [xot](https://github.com/faassen/xot)

[XML file]: ./data/content.xml

## Steps to reproduce

1. Clone this repo.
2. Run `cargo run` in the root directory of the repo.
3. Observe the error message.

## Expected behavior

The XML file should be successfully parsed, making the `unwrap` in the `main` function of the `src/main.rs` file not panic.

## Notes

The issue occurs with the released `xrust` crate version [`1.0.0`](https://crates.io/crates/xrust/1.0.0), as well as the latest commits on the `main` branch ([313aafd](https://github.com/ballsteve/xrust/commit/313aafd523b1f9fbfdfc4b961e6d7fe6b61c5db3)) and `dev` branch ([7fd30f9](https://github.com/ballsteve/xrust/commit/7fd30f935cf41b5a0f7fe11880646548dc52a5fa)) at the time of writing this README.
