# Quickset

```bash
$ quickset
Quickset v0.0.1 found local quickset file would you like to import it? [y/N]

$ quickset 
Quickset v0.0.1 importing .quickset.toml
Downloading []

```


```bash
$ quickset use onesupercoder/quickset #(tap and install)
$ quickset tap airbnb/quickset # tap make available 
#tap should be unnecessary because everything should have a tap
```


```toml
#designing format for quickset
#the idea is to allow all development dependencies to be setup

[dependencies]
rust = { tap = "airbnb/quickset", toolchain = "stable" }
sqlite = { version = "latest" }

[cargo]
diesel_cli = { depends-on = ["rust", "sqlite"], features = ["no-default-features", "features sqlite"] }
cargo-edit = {}
```