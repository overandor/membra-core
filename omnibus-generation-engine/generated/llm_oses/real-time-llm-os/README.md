# real-time-llm-os

Hard real-time scheduler guaranteeing inference latency bounds for safety-critical LLM outputs.

## Attributes
- **Kernel type**: real-time
- **LLM subsystem**: bounded_inference
- **Languages**: c, rust, ada

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
