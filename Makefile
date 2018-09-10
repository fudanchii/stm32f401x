.PHONY: app

app:
	rustup override set nightly
	xargo build --release
