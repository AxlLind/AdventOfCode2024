DAYS  := 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
TODAY := $(shell TZ=America/New_York date +%y%m%d)

# if today is an AOC-day set it as the default goal
.DEFAULT_GOAL := $(or $(filter $(TODAY:2412%=%),$(DAYS)),help)
.PHONY: $(DAYS) all help

inputs/%.in:
	./fetch.sh $*

src/bin/%.rs:
	DAY=$* envsubst < src/template.rs > $@

$(DAYS): %: inputs/%.in src/bin/%.rs
	cargo run --quiet --release --bin $*

all: $(patsubst src/bin/%.rs,inputs/%.in,$(wildcard src/bin/*.rs))
	cargo run --quiet --release

help:
	@echo 'usage: make [TARGET..]'
	@echo 'Automatically downloads input, sets up files, and runs solutions.'
	@echo
	@echo 'TARGET:'
	@echo '  {01..25}  run a specific day'
	@echo '  all       run all days'
	@echo "During the AoC month 'make' will run the current day's solution"
