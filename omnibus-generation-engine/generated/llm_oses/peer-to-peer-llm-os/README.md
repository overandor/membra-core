# peer-to-peer-llm-os

OS without central servers; LLM weights are erasure-coded across peer devices.

## Attributes
- **Kernel type**: p2p
- **LLM subsystem**: distributed_weights
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
