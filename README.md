# Nost Films

## About Nost

Nost, Note + Rust is a tool for keeping notes in markdown with special features and a dedicated format.

## Build the project

This project is binary project with rust.

You can easily build it with cargo.

```
cargo build --release 
```
A binary is build in target/release/nost-film

You can then move it and create an alias.

## Commands

### List all existing viewings in not

```
target/release/nost-film view
```

Or

```
target/release/nost-film v
```

It will return a list of movies with time like

```
In the mood for love à 15h00
Nine Perfect Strangers S0203 - 12:50
Holland - 22:36
```

### Add a viewing reference in not

Add a viewing reference in the last existing not file.

```
target/release/nost-film view <title> <optional hh:mm>
```

If you don't add a time, the current time will be added.
The valid format for time is <hh:mm>.