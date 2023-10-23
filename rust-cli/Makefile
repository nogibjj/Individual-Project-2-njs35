rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run

# Premade CRUD Operations:

create:
	cargo run -- "INSERT INTO books (Title, Author, Genre, Price, Stock) VALUES ('The Boys in the Boat', 'Daniel James Brown', 'Biography', 11.99, 25);"

read:
	cargo run -- "SELECT * FROM books WHERE Title = 'The Boys in the Boat';"

update:
	cargo run -- "UPDATE books SET Stock = 500 WHERE Title = 'The Boys in the Boat';"

delete:
	cargo run -- "DELETE FROM books WHERE Title = 'The Boys in the Boat';"
