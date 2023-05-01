mod tests

{
    use siri8080::i8080::registers::*;
    use siri8080::i8080::Processor;
    use rand::Rng;

    #[test]
    fn add()
    {
        let mut rng = rand::thread_rng();
        let mem = vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]; 
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

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
        
        let mut sum: u8 = 0;
        let mut carry: bool; 

        for i in 0..=6 {
            cpu.clock();
            let accumulator = cpu.get_registers().accumulator;
            (sum, carry)  = sum.overflowing_add(numbers[i]);
            let zero = sum == 0;
            let sign: bool = ((sum >> 7) & 0x1) == 0x1;
            let flags = cpu.get_flags();
            assert_eq!(flags.sign_flag, sign);
            assert_eq!(flags.carry_flag, carry);
            assert_eq!(flags.zero_flag, zero);
            assert_eq!(accumulator, sum);
        }
    }
}