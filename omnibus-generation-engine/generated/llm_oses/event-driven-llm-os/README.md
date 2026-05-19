# event-driven-llm-os

Kernel dispatch is entirely event-driven; LLM prompts are high-priority kernel events.

## Attributes
- **Kernel type**: event-driven
- **LLM subsystem**: event_router
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
