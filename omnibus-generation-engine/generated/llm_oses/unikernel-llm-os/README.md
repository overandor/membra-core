# unikernel-llm-os

Single-address-space unikernel optimized for single-tenant LLM inference on cloud hypervisors.

## Attributes
- **Kernel type**: unikernel
- **LLM subsystem**: monolithic_inference
- **Languages**: rust, c

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
