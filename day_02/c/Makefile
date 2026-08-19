CFLAGS = -std=c11 -Wall -Wextra -O2

day_02: day_02.c
	$(CC) $(CFLAGS) -o $@ $<

run: day_02
	./day_02 ../example_input.txt

clean:
	rm -f day_02

.PHONY: run clean
