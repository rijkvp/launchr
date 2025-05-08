# launchr

Application launcher, file search and dmenu replacement for Linux.

![screenshot](https://github.com/user-attachments/assets/32f0102c-8007-4ddf-b805-006677e1813f)

## Features

- Application launcher
- Super fast file search
- [dmenu](https://tools.suckless.org/dmenu/) replacement, dmenu-like run mode & scripting support

## Usage

```sh
launchr # dmenu like run mode
launchr -m apps # application launcher
launchr -m files # file search
echo options | launchr -d -p "Custom" # dmenu scripting
```

## Installation

### Using Nix flakes

```sh
nix profile install github:rijkvp/launchr
```

