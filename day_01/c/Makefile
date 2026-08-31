CC ?= cc
CFLAGS ?= -std=c99 -Wall -Wextra -O2

day_01: main.c
	$(CC) $(CFLAGS) -o day_01 main.c

run: day_01
	./day_01 ../input.txt

test: day_01
	@printf 'part 1: 11\npart 2: 31\n' > expected.txt
	@./day_01 ../test_input.txt > actual.txt
	@diff -u expected.txt actual.txt && echo "example test passed"
	@rm -f expected.txt actual.txt

clean:
	rm -f day_01 expected.txt actual.txt

.PHONY: run test clean
