ARCH ?= x86_64
TARGET = peer_to_peer_llm_os

.PHONY: all clean run

all:
	@echo "Build peer-to-peer-llm-os for $(ARCH)"
	@echo "Languages: rust, go, c"
	@echo "Kernel type: p2p"
	@echo "LLM subsystem: distributed_weights"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
