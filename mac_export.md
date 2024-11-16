To set up your Rust environment for compiling a macOS application using rustup and targeting macOS releases, follow these steps:
Step-by-Step Guide for macOS Releases

Install Rust: If you haven't installed Rust yet, you can do so using rustup:

bash

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

After installation, make sure your PATH is set correctly by following the instructions given by the installer.

Check Installed Targets: To check the available targets, you can run:

bash

    rustup target list
    
Install Apple SDK with these code(you can update SDK version by checking repo):

bash
    wget https://github.com/roblabla/MacOSX-SDKs/releases/download/macosx14.0/MacOSX14.0.sdk.tar.xz 
    tar -xf MacOSX14.0.sdk.tar.xz 
    export SDKROOT=$(pwd)/MacOSX14.0.sdk/
    export PATH=$PATH:~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/
    export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=rust-lld
    rustup target add x86_64-apple-darwin
    cargo build --release --target x86_64-apple-darwin

Add the macOS Target: By default, the macOS target (x86_64-apple-darwin) is usually included. However, if you want to specifically target other architectures (like ARM for M1/M2 chips), you can add them:

bash

    rustup target add x86_64-apple-darwin    # For Intel Macs
    rustup target add aarch64-apple-darwin   # For M1/M2 Macs

Set Up Your Cargo.toml: Make sure your Cargo.toml is configured correctly for your project and does not have any dependencies that are specific to other operating systems unless they are conditionally compiled.

Compile Your Code: Use cargo build with the --release flag to build your project for release. Specify the target if needed:

bash

    cargo build --release --target x86_64-apple-darwin    # For Intel Macs

Or for ARM:

bash

    cargo build --release --target aarch64-apple-darwin   # For M1/M2 Macs

Locate the Executable: After a successful build, the release executable will be located in the target/<target>/release/ directory (e.g., target/x86_64-apple-darwin/release/).