ARCH ?= x86_64
TARGET = event_driven_llm_os

.PHONY: all clean run

all:
	@echo "Build event-driven-llm-os for $(ARCH)"
	@echo "Languages: rust, c"
	@echo "Kernel type: event-driven"
	@echo "LLM subsystem: event_router"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
