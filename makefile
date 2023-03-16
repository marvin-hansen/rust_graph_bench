# Make will use bash instead of sh
SHELL := /usr/bin/env bash
# GNU make man page: http://www.gnu.org/software/make/manual/make.html

.PHONY: help
help:
	@echo ' '
	@echo '    make run   		Runs the quantum target defined in run.sh script.'


.PHONY: run
run:
	@source scripts/run.sh
