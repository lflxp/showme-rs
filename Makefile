run:
	cargo run -- monitor -l -c -N -d -s -i 1

scan:
	cargo run -- scan -i 127.0.0.1-255 -p 1-65535

server:
	cargo run -- server -h 0.0.0.0 -t tokio -p 9999

fzf:
	cargo run --bin clapdemo -- fzf

wrap:
	@echo curl http://127.0.0.1:3030/ex/src/main.rs
	@echo curl -H "Content-Type:application/json" -X POST -d '{"service":10,"partId":"2354325235","name":"nameSean","rate":999}' 'http://127.0.0.1:3030/employees/999'
	cargo run -- server --type wrap