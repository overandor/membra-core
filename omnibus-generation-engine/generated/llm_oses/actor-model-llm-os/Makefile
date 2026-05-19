ARCH ?= x86_64
TARGET = actor_model_llm_os

.PHONY: all clean run

all:
	@echo "Build actor-model-llm-os for $(ARCH)"
	@echo "Languages: rust, erlang-vm"
	@echo "Kernel type: actor-model"
	@echo "LLM subsystem: token_actor"

clean:
	rm -rf build/

run: all
	qemu-system-$(ARCH) -kernel build/$(TARGET).bin
