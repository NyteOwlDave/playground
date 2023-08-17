
# Publih Crate

Execute this:

```
cargo publish
```

Before publishing, you need to establish a unique
crate name. Pick a tenative name, then search for
it here to verify uniqueness.

[crates]: <https://crates.io/>
[profile]: <https://crates.io/settings/profile>
[crt]: <https://doc.rust-lang.org/cargo/reference/config.html?highlight=CARGO_REGISTRY_TOKEN#credentials>

[Crate Repo][crates]<br>

Once a name has been established, you need an API token.

You can obtain an API token here:

[Profile][profile]<br>

Next, run the below to activate the API token.

```
cargo login
```

This API token is for local development.

For CI (continuous integration), you can use
`CARGO_REGISTRY_TOKEN` as described here:

[Credentials][crt]<br>

# Meta Data

Your project can't be published without certain metadata
in `Cargo.toml`.

```
[package]
name="uniquename"
license="MIT"
description="Dave's Crate"
```

Where <name> is the unique name chosen and verified earlier.

# References

[trpl]: <https://read.amazon.com/?asin=B071YKRV8Q&ref_=kwl_kr_iv_rec_1&language=en-US>

[TRPL Chapter 14][trpl]<br>

