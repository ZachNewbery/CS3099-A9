# How to set up backend on Ubuntu
This guide assumes that you already have a database set up with the host servers. If not, consult the Systems wiki.

Also, please, don't try this on Windows.

### Downloading dependencies
1. Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. MySQL: [Visit documentation](https://dev.mysql.com/doc/mysql-installation-excerpt/5.7/en/linux-installation-apt-repo.html)
3. Diesel: `cargo install diesel_cli --no-default-features --features mysql`

### Set up your connection
- Create a tunnel **(do this in a separate terminal!)**:
```shell script
echo "ssh username@username.host.cs.st-andrews.ac.uk -L 3306:localhost:3306 -N" > database.sh
./database.sh
```
- Set up `.env`
```shell script
echo "DATABASE_URL=mysql://username:DATABASE_PASSWORD@127.0.0.1/username_somethinghere" > .env
```

### Initialise Diesel
```
diesel setup
diesel migration run
```

### Ready, set, go!
```
cargo run --release
```
