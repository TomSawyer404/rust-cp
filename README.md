# rust-cp

The second utility you will build is called rust-cp, a variant of the UNIX tool cp. This tool copys a file to another file.

Here is how we use rust-cp:

```text
WSL rust-cp on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯ cargo r Cargo.toml new_file
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-cp Cargo.toml new_file`
         >>> Copy Done!

WSL rust-cp on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯ diff Cargo.toml new_file

WSL rust-cp on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯  

```

