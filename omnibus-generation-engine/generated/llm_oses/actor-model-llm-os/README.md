# actor-model-llm-os

Every process is an actor; LLM tokens are messages with backpressure and supervision trees.

## Attributes
- **Kernel type**: actor-model
- **LLM subsystem**: token_actor
- **Languages**: rust, erlang-vm

## Architecture

```
bootloader/   → minimal stub or GRUB multiboot
kernel/       → kernel main, scheduler, memory manager
llm_subsystem/→ token inference, KV-cache, model loader
drivers/      → UART, framebuffer, NVMe, GPU
```

## Build

```bash
make
```

## Run in QEMU

```bash
make run ARCH=x86_64
```

## License
MIT
