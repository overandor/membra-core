ARCH ?= x86_64
TARGET = mobile_llm_os

.PHONY: all clean run

all:
	@echo "Build mobile-llm-os for $(ARCH)"
	@echo "Languages: rust, c, cpp"
	@echo "Kernel type: mobile"
	@echo "LLM subsystem: thermal_quantizer"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
