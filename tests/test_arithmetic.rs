mod tests

{
    use siri8080::i8080::registers::*;
    use siri8080::i8080::Processor;

    #[test]
    fn add()
    {
        let mem = vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]; 
        let numbers = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            accumulator: 0,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5],
        };

        cpu.set_all_registers(regs);

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, numbers[6]);
        
        let mut sum = 0;
        for i in 0..=6 {
            cpu.clock();
            let accumulator = cpu.get_registers().accumulator;
            sum += numbers[i];
            println!("i {}", i);
            println!("accumulator {} sum {}", accumulator, sum);
            assert_eq!(accumulator, sum);
        }
    }
}
