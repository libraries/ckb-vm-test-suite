use ckb_vm::{
    machine::{trace::TraceMachine, DefaultCoreMachine, DefaultMachineBuilder, VERSION2},
    memory::wxorx::WXorXMemory,
    Bytes, SparseMemory, ISA_A, ISA_B, ISA_IMC, ISA_MOP,
};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = std::fs::read(args[0].clone()).unwrap().into();
    let args: Vec<Bytes> = args.into_iter().map(|a| a.into()).collect();

    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_A | ISA_B | ISA_MOP,
        VERSION2,
        u64::MAX,
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    machine.load_program(&code, &args).unwrap();
    let result = machine.run();
    if result != Ok(0) {
        println!("Error: {:?}", result);
        exit(i32::from(result.unwrap_or(-1)));
    }
}
