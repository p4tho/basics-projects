# rust operating system
Learn about operating system internals with Philipp Oppermann's blog (https://os.phil-opp.com/) that walks through how to build an operating system with Rust.

## how to use
1) ```cargo bootimage``` to compile kernel and bootloader then link them to create a bootimage
2) ```qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-operating-system.bin``` to boot up a VM with bootimage
3) Or just use ```cargo run``` since configured to do these steps with cargo's run command