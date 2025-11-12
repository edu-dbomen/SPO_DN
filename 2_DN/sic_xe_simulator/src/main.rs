mod machine;

use machine::Machine;

fn main() {
    let mut machine = Machine::new();
    machine.get_device(1).write(0x48);
    machine.get_device(1).write(0x45);
    machine.get_device(1).write(0x4C);
    machine.get_device(1).write(0x4C);
    machine.get_device(1).write(0x4F);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x0A);

    let _ = machine.get_device(0).read();

    machine.memory.set_byte(0xabcd, 0x48);
    machine.memory.set_byte(0xabce, 0x49);
    machine.memory.set_byte(0xabcf, 0x0A);
    let word = machine.memory.get_word(0xabcd);
    for byte in word {
        machine.get_device(1).write(byte.clone());
    }

    machine.get_device(1).write(0x41);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x20);
    let val_a = machine.registers.get_a() as i8;
    machine.get_device(1).write(val_a);
    machine.get_device(1).write(0x0A);

    machine.registers.set_a(69);
    machine.get_device(1).write(0x41);
    machine.get_device(1).write(0x3A);
    machine.get_device(1).write(0x20);
    let val_a = machine.registers.get_a() as i8;
    machine.get_device(1).write(val_a);
    machine.get_device(1).write(0x0A);
}
