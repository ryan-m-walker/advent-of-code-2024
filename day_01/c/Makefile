CFLAGS = -std=c11 -Wall -Wextra -O2

day_01: day_01.c
	$(CC) $(CFLAGS) -o day_01 day_01.c

.PHONY: run test clean

run: day_01
	./day_01 ../input.txt

test: day_01
	./day_01 ../test_input.txt

clean:
	rm -f day_01
