# Partial mixed markdown

If you're looking for a markdown parser and renderer you probably want
[comark](https://github.com/kivikakk/comrak).

Convert a subset of markdown mixed with some html to html:

```markdown
Header 1
========

## Header 2

Paragraph one
on one line.

<style>
h1 {
  color: red;
}
</style>

Paragraph two!
```

produces

```html
<h1>Header 1</h1>
<h2>Header 2</h2>
<p>Paragraph one on one line.</p>
<style>
h1 {
  color: red;
}
</style>
<p>Paragraph two!</p>
```

**Note:** Currently only markdown headings and the html style tag are supported.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
