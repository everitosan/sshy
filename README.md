![Rust](https://img.shields.io/badge/Rust:1.81-black?style=for-the-badge&logo=rust&logoColor=#E57324)
![Rust](https://img.shields.io/badge/Sqlite:1.70-blue?style=for-the-badge&logo=sqlite&logoColor=#E57324)
# SSHY
SSH connections manager without config files.

## Requirements

`ssh`, `openssl` and `ssh-keygen` must be installed in your system.

## Features  
- Manage groups, severs and credentials
- Connect to registered servers
- Remote execution of scripts similar to [cry](https://github.com/everitosan/BashScripts/tree/main/cri) 
- Portable and Encrypted information source (sqlcipher)

**Wip**
- Group edition and delete 
- Server edition and delete 

## Install

To install via cargo run the following command

```bash
cargo install --git https://github.com/everitosan/sshy.git --branch stable
```

## Guides
- [Development](./README/dev.md)
