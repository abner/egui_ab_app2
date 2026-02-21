## Template Egui App for Use for Internal Desktop Tooling



## Development

```bash
cargo run
```


### Package Bundle

Install cargo bundle:

```bash
cargo install cargo-bundle
```

Bundle the app:

```bash
cargo bundle --release
```



# How to create assets/icon.icns

1. Used online tool https://www.appicon.co/ to generate the set of icons

2. Save the png with 1024x1024 size into the directory assets with the name 1024.png.

3. Execute the script 

```bash
./scripts/png2icns.sh assets/1024.png ./assets/icons
```

An iconset file called `icons.icns` will be created within the `assets/` directory.


# Cross compile to Windows

Install cargo-zigbuild.

```bash
cargo install cargo-zigbuild
```

## Compile to Windows

### Cargo Zig Build

```bash
cargo zigbuild --target x86_64-pc-windows-gnu -r
```

A binary file will be geenrated into `target/release/x86_64-pc-windows-gnu`



### Cargo XWin

> It does generate the executable, but I coudln't generate an installer

For using windows-msvc

```bash
cargo install --locked cargo-xwin`
```

ant then:

```bash
cargo xwin build --target x86_64-pc-windows-msvc
```

### Kodegen Bundler Bundle

[kodegen-bundler-bundle

](https://github.com/cyrup-ai/kodegen-bundler-bundle/tree/main)

It requires the assets/img/icon.ico icon to be in this exact path so it would be used in the
generated app for be deployed via the installer.

https://github.com/cyrup-ai/kodegen-bundler-bundle/blob/main/README.md?plain=1#L327

You can create it using this website: https://www.freeconvert.com/png-to-ico/

This command will use Docker to build the windows installer:

```bash
kodegen_bundler_bundle  --source abner/egui_ab_app --platform exe --output-binary ./ab-app-install.exe
```