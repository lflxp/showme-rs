run:
	cargo run -- monitor -l -c -N -d -s -i 1

scan:
	cargo run -- scan -i 127.0.0.1-255 -p 1-65535

server:
	cargo run -- server -h 0.0.0.0 -t tokio -p 9999