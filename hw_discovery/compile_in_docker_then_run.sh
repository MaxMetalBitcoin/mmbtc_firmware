# cargo install cross # needed to compile for the hw in docker
cross build --target thumbv7em-none-eabihf &&

# to run - copied from the runner in .cargo/config.toml
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/groundup -q -x ./openocd.gdb