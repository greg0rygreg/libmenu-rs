# libmenu-rs
remake of [libmenu](https://github.com/greg0rygreg/libmenu) in rust

## making your own app
1. clone this repo

2. rename `lib` to `libmenu-rs` and one of the example folders to anything you want

3. if you chose to rename an example folder in step 2, delete the remaining folders

4. in the `Cargo.toml` file of the parent directory, replace:

```toml
members = [ "lib", "example", ... ]
```

with:

```toml
members = [ "libmenu-rs", "your_app_name" ]
```

and replace `your_app_name` with the new name of the just-renamed `example` folder

5. in `your_app_name/Cargo.toml`, replace:

```toml
libmenu-rs = { path = "../lib" }
```

with:

```toml
libmenu-rs = { path = "../libmenu-rs" }
```

6. in the parent directory, run your app with:

```sh
cargo run -p your_app_name --release
```