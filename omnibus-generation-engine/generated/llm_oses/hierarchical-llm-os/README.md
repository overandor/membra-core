# hierarchical-llm-os

Ring-0 hosts small model, delegates to ring-1 medium, ring-2 large model via hierarchical calls.

## Attributes
- **Kernel type**: hierarchical
- **LLM subsystem**: tiered_model
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
