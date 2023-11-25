mod tests

{
    use siri8080::i8080::registers::*; 
    use siri8080::i8080::Processor;
    use rand::Rng;

    #[test]
    fn add()
    {
        let mut rng = rand::thread_rng();
        let mem = vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]; 
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers 
        {
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

        for i in 0..=7 
        {
            cpu.clock();
            let accumulator = cpu.get_registers().accumulator;

            if i == 7
            {
                (sum, carry) = sum.overflowing_add(sum);
            }
            else
            {
                (sum, carry) = sum.overflowing_add(numbers[i]);
            }

            let zero        = sum == 0;
            let sign        = ((sum >> 7) & 0x1) == 0x1;
            let parity      = sum.count_ones() % 2 != 0;
            let flags       = cpu.get_flags();
            assert_eq!(flags.sign_flag, sign);
            assert_eq!(flags.carry_flag, carry);
            assert_eq!(flags.zero_flag, zero);
            assert_eq!(flags.parity_flag, parity);
            assert_eq!(accumulator, sum);
        }
    }

    #[test]
    fn adc()
    {
        let mut rng = rand::thread_rng();
        let mem = vec![0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 
                       0x8f]; 
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers 
        {
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
        let mut carry: bool = false; 

        for i in 0..=7 
        {
            cpu.clock();
            let accumulator = cpu.get_registers().accumulator;
            sum += carry as u8;

            if i == 7
            {
                (sum, carry) = sum.overflowing_add(sum - (carry as u8));
            }
            else
            {
                (sum, carry) = sum.overflowing_add(numbers[i]);
            }

            let zero     = sum == 0;
            let sign     = ((sum >> 7) & 0x1) == 0x1;
            let flags    = cpu.get_flags();
            let parity      = sum.count_ones() % 2 != 0;
            assert_eq!(accumulator, sum);
            assert_eq!(flags.parity_flag, parity);
            assert_eq!(flags.sign_flag, sign);
            assert_eq!(flags.carry_flag, carry);
            assert_eq!(flags.zero_flag, zero);
        }
    }

    #[test]
    fn sub()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mem     = vec![0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 0,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);
        let addr = (reg.h as u16) << 8 | (reg.l as u16);
        cpu.set_memory_at(addr, numbers[6]);

        let mut sum: u8 = 0;
        let mut carry: bool;

        for i in 0..= 7
        {
            cpu.clock();
            if i == 7
            {
                (sum, carry) = sum.overflowing_sub(sum);
            }
            else
            {
                (sum, carry) = sum.overflowing_sub(numbers[i]);
            }
            let regs        = cpu.get_registers();
            let flags       = cpu.get_flags();
            let zero        = sum == 0;
            let sign: bool  = ((sum >> 7) & 0x1) == 0x1;
            let parity      = sum.count_ones() % 2 != 0;

            assert_eq!(flags.parity_flag, parity);
            assert_eq!(sum, regs.accumulator);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(carry, flags.carry_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

    #[test]
    fn sbb()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..8)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mem     = vec![0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 0,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);
        let addr = (reg.h as u16) << 8 | (reg.l as u16);
        cpu.set_memory_at(addr, numbers[6]);

        let mut sum: u8 = 0;
        let mut carry: bool = false;

        for i in 0..= 4
        {
            cpu.clock();
            sum = sum - (carry as u8);
            if i == 7
            {
                (sum, carry) = sum.overflowing_sub(sum - (carry as u8));
            }
            else
            {
                (sum, carry) = sum.overflowing_sub(numbers[i]);
            }
            let regs        = cpu.get_registers();
            let flags       = cpu.get_flags();
            let zero        = sum == 0;
            let sign: bool  = ((sum >> 7) & 0x1) == 0x1;
            let parity      = sum.count_ones() % 2 != 0;

            assert_eq!(flags.parity_flag, parity);
            assert_eq!(sum, regs.accumulator);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(carry, flags.carry_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

    #[test]
    fn ana()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mem     = vec![0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 0,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);
        let addr = (reg.h as u16) << 8 | (reg.l as u16);
        cpu.set_memory_at(addr, numbers[6]);

        let mut sum: u8 = 0;
        let carry: bool = false;

        for i in 0..= 7
        {
            cpu.clock();

            if i == 7
            {
                sum = sum & sum;
            }
            else
            {
                sum = sum & numbers[i];
            }
            let regs       = cpu.get_registers();
            let flags      = cpu.get_flags();
            let zero       = sum == 0;
            let sign: bool = ((sum >> 7) & 0x1) == 0x1;
            let parity     = sum.count_ones() % 2 != 0;

            assert_eq!(flags.parity_flag, parity);
            assert_eq!(sum, regs.accumulator);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(carry, flags.carry_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

    #[test]
    fn xra()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mem     = vec![0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 0,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);
        let addr = (reg.h as u16) << 8 | (reg.l as u16);
        cpu.set_memory_at(addr, numbers[6]);

        let mut sum: u8 = 0;
        let carry: bool = false;

        for i in 0..= 7
        {
            cpu.clock();

            if i == 7
            {
                sum = sum ^ sum;
            }
            else
            {
                sum = sum ^ numbers[i];
            }
            let regs       = cpu.get_registers();
            let flags      = cpu.get_flags();
            let zero       = sum == 0;
            let sign: bool = ((sum >> 7) & 0x1) == 0x1;
            let parity     = sum.count_ones() % 2 != 0;

            assert_eq!(flags.parity_flag, parity);
            assert_eq!(sum, regs.accumulator);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(carry, flags.carry_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

    #[test]
    fn ora()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mem     = vec![0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 0,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);
        let addr = (reg.h as u16) << 8 | (reg.l as u16);
        cpu.set_memory_at(addr, numbers[6]);

        let mut sum: u8 = 0;
        let carry: bool = false;

        for i in 0..= 7
        {
            cpu.clock();

            if i == 7
            {
                sum = sum | sum;
            }
            else
            {
                sum = sum | numbers[i];
            }
            let regs       = cpu.get_registers();
            let flags      = cpu.get_flags();
            let zero       = sum == 0;
            let sign: bool = ((sum >> 7) & 0x1) == 0x1;
            let parity     = sum.count_ones() % 2 != 0;

            assert_eq!(sum, regs.accumulator);
            assert_eq!(flags.parity_flag, parity);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(carry, flags.carry_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

    #[test]
    fn cmp()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mem     = vec![0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 10,
            b: numbers[0],
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);
        let addr = (reg.h as u16) << 8 | (reg.l as u16);
        cpu.set_memory_at(addr, numbers[6]);

        let mut carry: bool;
        let mut tmp: u8;

        for i in 0..= 7
        {
            cpu.clock();

            if i == 7
            {
                (tmp, carry) = reg.accumulator.overflowing_sub(reg.accumulator);
            }
            else
            {
                (tmp, carry) = reg.accumulator.overflowing_sub(numbers[i]);
            }
            let flags      = cpu.get_flags();
            let zero       = tmp == 0;
            let sign: bool = ((tmp >> 7) & 0x1) == 0x1;
            let parity     = tmp.count_ones() % 2 != 0;

            assert_eq!(flags.parity_flag, parity);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(carry, flags.carry_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

    #[test]
    fn inr()
    {
        let mut rng = rand::thread_rng();
        let numbers: Vec<u8> = (0..7)
            .map(|_| rng.gen_range(1..255))
            .collect();

        let mut numbers_copy = numbers.clone();
        let mem     = vec![0x04, 0x0c, 0x14, 0x1c, 0x24, 0x2c, 0x34, 0x3c];
        let mut cpu = Processor::from_bytes(mem);
        let reg     = Registers
        { 
            accumulator: 0,
            b: numbers[0].clone(),
            c: numbers[1],
            d: numbers[2],
            e: numbers[3],
            h: numbers[4],
            l: numbers[5]
        };

        cpu.set_all_registers(reg);

        for i in 0..= 6
        {
            let reg = cpu.get_registers();
            let addr = (reg.h as u16) << 8 | (reg.l as u16);
            cpu.set_memory_at(addr, numbers[6]);

            cpu.clock();

            (numbers_copy[i], _) = numbers[i].overflowing_add(1);
            let flags      = cpu.get_flags();
            let zero       = numbers_copy[i] == 0;
            let sign: bool = ((numbers_copy[i] >> 7) & 0x1) == 0x1;
            let parity     = numbers_copy[i].count_ones() % 2 != 0;
            let mem_value  = cpu.get_memory_at(addr);

            let reg = cpu.get_registers();
            assert_eq!(numbers_copy[0], reg.b);
            assert_eq!(numbers_copy[1], reg.c);
            assert_eq!(numbers_copy[2], reg.d);
            assert_eq!(numbers_copy[3], reg.e);
            assert_eq!(numbers_copy[4], reg.h);
            assert_eq!(numbers_copy[5], reg.l);
            assert_eq!(numbers_copy[5], reg.l);
            assert_eq!(numbers_copy[6], mem_value);

            assert_eq!(flags.parity_flag, parity);
            assert_eq!(zero, flags.zero_flag);
            assert_eq!(sign, flags.sign_flag);
        }
    }

}
