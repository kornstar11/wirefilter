# Wirefilter

[![Build status](https://img.shields.io/travis/com/cloudflare/wirefilter/master.svg)](https://travis-ci.com/cloudflare/wirefilter)
[![Crates.io](https://img.shields.io/crates/v/wirefilter-engine.svg)](https://crates.io/crates/wirefilter-engine)
[![License](https://img.shields.io/github/license/cloudflare/wirefilter.svg)](LICENSE)

This is an execution engine for [Wireshark®](https://www.wireshark.org/)-like filters.

It contains public APIs for parsing filter syntax, compiling them into
an executable IR and, finally, executing filters against provided values.

## Example

```rust
use wirefilter::{ExecutionContext, Scheme, Type};

fn main() -> Result<(), failure::Error> {
    // Create a map of possible filter fields.
    let scheme = Scheme! {
        http.method: Bytes,
        http.ua: Bytes,
        port: Int,
    };

    // Parse a Wireshark-like expression into an AST.
    let ast = scheme.parse(r#"
        http.method != "POST" &&
        not http.ua matches "(googlebot|facebook)" &&
        port in {80 443}
    "#)?;

    println!("Parsed filter representation: {:?}", ast);

    // Compile the AST into an executable filter.
    let filter = ast.compile();

    // Set runtime field values to test the filter against.
    let mut ctx = ExecutionContext::new(&scheme);

    ctx.set_field_value("http.method", "GET")?;

    ctx.set_field_value(
        "http.ua",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:66.0) Gecko/20100101 Firefox/66.0",
    )?;

    ctx.set_field_value("port", 443)?;

    // Execute the filter with given runtime values.
    println!("Filter matches: {:?}", filter.execute(&ctx)?); // true

    // Amend one of the runtime values and execute the filter again.
    ctx.set_field_value("port", 8080)?;

    println!("Filter matches: {:?}", filter.execute(&ctx)?); // false

    Ok(())
}
```

## Macros Example
Using derive macros you can create a domain struct and auto genereate the Scheme and filter logic. See below:

Defining our domain objects:
```rust
#[derive(Debug, Filterable, HasFields)]
#[field(name="http")]
struct Http {
    method: String,
    ua: i32,
}
```

```rust
#[derive(Debug, Filterable, HasFields)]
struct Flow {
    port: i32
}
```

* `Filterable` will impl the Filterable trait which takes a Scheme and returns a populated `Result<ExecutionContext, Error>`
* `HasFields` will create a `fields()` static method which returns a `Vec<(String, Type)>`. This vec can be used to create a Scheme using the `try_from_iter` method.

Putting it together we can do the following: 

```rust
#[derive(Debug, Filterable, HasFields)]
#[field(name="http")]
struct Http {
    method: String,
    ua: String,
}

#[derive(Debug, Filterable, HasFields)]
struct Flow {
    port: i32
}
let fields = Http::fields().extend(Flow::fields());
let scheme = Scheme::try_from_iter(fields.into_iter())?;

// Parse a Wireshark-like expression into an AST.
let ast = scheme.parse(r#"
    http.method != "POST" &&
    not http.ua matches "(googlebot|facebook)" &&
    port in {80 443}
"#)?;

println!("Parsed filter representation: {:?}", ast);

// Compile the AST into an executable filter.
let filter = ast.compile();

let http = Http { 
    method: String::from("GET"),
    ua: "Mozilla"
};

let http_context = http.filter_context(&scheme);
let result = filter.execute(&http_context)?;

println!("Result {}", result);

```
## Licensing

Licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
