# paster

Easily paste from your terminal to services like pastebin.com

## Features

- Automatic format detection (WIP)
- Multiple paste destinations (WIP)
- Easy to configure

## How to install

### Homebrew

```bash
brew install paster
```

### Cargo

```bash
cargo install paster
```

## How to use it

### Set your key

Paster comes with pastebin.com configured as a default destination. All you
need to get started is to write your key to the config.

```bash
paster config dest.pastebin.dev_key <your pastebin.com developer API key>
```

### Create paste from stdin

```bash
echo Hello | paster
```

### Create paste from file

```bash
paster file.txt
```

### Paste to non-default destination

```bash
paster -d other example.rs
```

