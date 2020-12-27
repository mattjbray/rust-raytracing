image.png: image.ppm
	convert $< $@


clean:
	rm image.png image.ppm

image.ppm: src/main.rs
	cargo run > $@
