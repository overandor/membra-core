# exokernel-llm-os

Exokernel exposing raw GPU/TPU hardware to user-space LLM runtimes with secure multiplexing.

## Attributes
- **Kernel type**: exokernel
- **LLM subsystem**: hardware_multiplexer
- **Languages**: rust, c, cpp

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
