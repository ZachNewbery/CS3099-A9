# How to set up backend on Ubuntu
This guide assumes that you already have a database set up with the host servers. If not, consult the Systems wiki.

Also, please, don't try this on Windows.

### Downloading dependencies
1. Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. MySQL: [Visit documentation](https://dev.mysql.com/doc/mysql-installation-excerpt/5.7/en/linux-installation-apt-repo.html)
3. Diesel: `cargo install diesel_cli --no-default-features --features mysql`

### Set up your connection
- Set up JWT Secret Key
```shell script
head -c16 /dev/urandom > jwt_secret.key
```
- Set up `.env`, replacing the relevant variables. The below statement assumes you have a tunnel setup on port 3307, but change as necessary.
```shell script
echo "DATABASE_URL=mysql://<DATABASE_USERNAME>:<DATABASE_PASSWORD>@localhost:3307/<DATABASE_NAME>" > .env
```

### Compile and Build Project
The following command(s) will update and resolve any dependencies and build the program:
```
cargo check && cargo build
```
If on host servers, due to resource constraints you will need to export the following environment variables:

```shell script
export ACTIX_THREADPOOL=16
```
### Ready, set, go!
Run the following to run the backend program on port 8000:
```
cargo run --release
```
Alternatively, a pre-compiled binary has been setup in the backend root folder, and can be run using ./starlight
