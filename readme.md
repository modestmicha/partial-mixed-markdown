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
