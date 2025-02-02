# Rust-Todo-CLI

Commands:

add :
cargo run -- add "task"

delete :
cargo run -- delete {taskNumber}

list :
cargo run -- list

done :
cargo run -- done {taskNumber}

Example:
cargo run -- add "First task"
cargo run -- list
cargo run -- done 1
cargo run -- delete 1
cargo run -- list
