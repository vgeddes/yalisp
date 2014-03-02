bin := src/main
src := src/main.rs

all: $(bin)

$(bin): $(src)
	rustc -g $< -o $@

clean:
	rm -f $(bin)
