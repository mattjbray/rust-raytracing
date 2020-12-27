image.png: image.ppm
	convert $< $@


clean:
	rm -f image.png image.ppm

image.ppm: src/main.rs
	cargo run --release > $@
