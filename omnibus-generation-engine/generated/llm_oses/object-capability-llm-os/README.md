# object-capability-llm-os

Object-capability security; each LLM token generation requires a capability delegation chain.

## Attributes
- **Kernel type**: object-capability
- **LLM subsystem**: delegated_generation
- **Languages**: rust, javascript

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
