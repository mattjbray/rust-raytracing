image.png: image.ppm
	convert $< $@


clean:
	rm -f image.png image.ppm

image.ppm: src/*.rs
	cargo run --release > $@
