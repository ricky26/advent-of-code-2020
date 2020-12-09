use std::io::Read;
use std::collections::BTreeSet;
use aoc2020::vm::{VM, Instruction};
use aoc2020::asm::parse_asm;

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let asm = parse_asm(&contents)?.1;
    let mut seen = BTreeSet::new();

    let mut contenders = Vec::new();

    let mut vm = VM::new(asm, Default::default());
    while seen.insert(vm.state().ip) {
        let ip = vm.state().ip;
        if let Some(Instruction::Nop(_)) = vm.instructions().get(ip as usize) {
            contenders.push(ip);
        }

        vm.step()?;
        let nip = vm.state().ip;
        if nip != ip + 1 {
            contenders.push(ip);
        }
    }

    println!("acc {} ip {}", vm.state().acc, vm.state().ip);

    for contender in contenders {
        let contender = contender as usize;
        let mut inst = vm.instructions().to_vec();

        match inst[contender] {
            Instruction::Nop(x) => inst[contender] = Instruction::Jmp(x),
            Instruction::Jmp(x) => inst[contender] = Instruction::Nop(x),
            _ => unreachable!(),
        }

        println!("trying {}", contender);

        let mut seen = BTreeSet::new();
        let mut vm = VM::new(inst, Default::default());
        while seen.insert(vm.state().ip) {
            let ip = vm.state().ip;
            if ip == vm.instructions().len() as i32 {
                println!("it works {}, acc {}", contender, vm.state().acc);
                break;
            }

            vm.step()?;
        }
    }

    Ok(())
}
