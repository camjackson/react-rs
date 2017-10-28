[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

# react-rs
React library for server-side rendering with Rust. Very early days, this doesn't do much yet.

When it's done, this:
```rust
react.Div { 
  className: "hello",
  children: vec![react.Img { src: "world.jpg" }]
}.render()
```

will return this:
```html
<div class="hello"><img src="world.jpg"/></div>
```

However, Instead of writing that very verbose Rust, you'll be able to use [jsx-rs](https://github.com/camjackson/jsx-rs).
