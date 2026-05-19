# serverless-llm-os

Cold-start optimized kernel for ephemeral LLM functions; keeps hot model weights in kernel-resident cache.

## Attributes
- **Kernel type**: serverless
- **LLM subsystem**: hot_weight_cache
- **Languages**: rust, go, c

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
