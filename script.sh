##
cargo add serde serde_json --features derive

cargo add validator --features derive

#fo read configuration ENV (not only env)
cargo add config

#time time
cargo add chrono

#asyc runtime
cargo add tokio --features full

#db
cargo add sqlx --features runtime-tokio,chrono,mysql

cargo add actix-web actix-rt

