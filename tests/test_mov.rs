mod tests

{
    // use siri8080::i8080::instructions::{InstructionTypes, Instruction};
    use siri8080::i8080::registers::*;
    // use siri8080::i8080::flags::*;
    use siri8080::i8080::Processor;
    use rand::Rng;

    #[test]
    fn mvi_d8() 
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(1..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = 
            vec![0x06, randoms[0], 0x0E, randoms[1], 
            0x16, randoms[2], 0x1E, randoms[3], 0x26,
            randoms[4], 0x2E, randoms[5], 0x36, randoms[6], 0x3E, randoms[7]];

        let mut cpu = Processor::from_bytes(mem);

        for _ in 0..8 {
            cpu.clock();
        }

        let regs = cpu.get_registers();
        let memory_ref = (regs.h as u16) << 8 | (regs.l as u16);
        assert_eq!(regs.b, randoms[0]);
        assert_eq!(regs.c, randoms[1]);
        assert_eq!(regs.d, randoms[2]);
        assert_eq!(regs.e, randoms[3]);
        assert_eq!(regs.h, randoms[4]);
        assert_eq!(regs.l, randoms[5]);
        assert_eq!(cpu.get_memory_at(memory_ref), randoms[6]);
        assert_eq!(regs.accumulator, randoms[7]);
    }

    #[test]
    fn mov_a()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x7F, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            accumulator: randoms[0],
            b: randoms[1],
            c: randoms[2],
            d: randoms[3],
            e: randoms[4],
            h: randoms[5],
            l: randoms[6],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[7]);
        cpu.set_all_registers(regs);

        for i in 0..=7 
        {
            cpu.clock();
            let regs = cpu.get_registers();
            assert_eq!(regs.accumulator, randoms[i]);
        }
    }

    #[test]
    fn mov_b()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[0],
            c: randoms[1],
            d: randoms[2],
            e: randoms[3],
            h: randoms[4],
            l: randoms[5],
            accumulator: randoms[7],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[6]);
        cpu.set_all_registers(regs);

        for i in 0..=7
        {
            cpu.clock();
            let regs = cpu.get_registers();
            assert_eq!(regs.b, randoms[i]);
        }
    }


    #[test]
    fn mov_c()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x49, 0x48 , 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[1],
            c: randoms[0],
            d: randoms[2],
            e: randoms[3],
            h: randoms[4],
            l: randoms[5],
            accumulator: randoms[7],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[6]);
        cpu.set_all_registers(regs);

        for i in 0..=7
        {
            cpu.clock();
            let regs = cpu.get_registers();
            println!("{} {} {}", regs.c, randoms[i], i);
            assert_eq!(regs.c, randoms[i]);
        }

    }

    #[test]
    fn mov_d()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x52, 0x50, 0x51, 0x53, 0x54, 0x55, 0x56, 0x57];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[1],
            c: randoms[2],
            d: randoms[0],
            e: randoms[3],
            h: randoms[4],
            l: randoms[5],
            accumulator: randoms[7],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[6]);
        cpu.set_all_registers(regs);

        for i in 0..=7
        {
            cpu.clock();
            let regs = cpu.get_registers();
            assert_eq!(regs.d, randoms[i]);
        }
    }

    #[test]
    fn mov_e()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x5B, 0x58, 0x59, 0x5A, 0x5C, 0x5D, 0x5E, 0x5F];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[1],
            c: randoms[2],
            d: randoms[3],
            e: randoms[0],
            h: randoms[4],
            l: randoms[5],
            accumulator: randoms[7],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[6]);
        cpu.set_all_registers(regs);

        for i in 0..=7
        {
            cpu.clock();
            let regs = cpu.get_registers();
            assert_eq!(regs.e, randoms[i]);
        }
    }

    #[test]
    fn mov_h()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x64, 0x66, 0x60, 0x61, 0x62, 0x63, 0x65, 0x67];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[2],
            c: randoms[3],
            d: randoms[4],
            e: randoms[5],
            h: randoms[0],
            l: randoms[6],
            accumulator: randoms[7],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[1]);
        cpu.set_all_registers(regs);

        for i in 0..=7
        {
            cpu.clock();
            let regs = cpu.get_registers();
            println!("{} {} {}", regs.h, randoms[i], i);
            assert_eq!(regs.h, randoms[i]);
        }
    }

    #[test]
    fn mov_l()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..8)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x6D, 0x6E, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6F];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[2],
            c: randoms[3],
            d: randoms[4],
            e: randoms[5],
            h: randoms[6],
            l: randoms[0],
            accumulator: randoms[7],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[1]);
        cpu.set_all_registers(regs);

        for i in 0..=7
        {
            cpu.clock();
            let regs = cpu.get_registers();
            assert_eq!(regs.l, randoms[i]);
        }
    }

    #[test]
    fn mov_m()
    {
        let mut rng = rand::thread_rng();
        let randoms = (0..7)
            .map(|_| rng.gen_range(0..255))
            .collect::<Vec<u8>>();

        let mem: Vec<u8> = vec![0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x77];
        let mut cpu = Processor::from_bytes(mem);
        let regs = Registers {
            b: randoms[0],
            c: randoms[1],
            d: randoms[2],
            e: randoms[3],
            h: randoms[4],
            l: randoms[5],
            accumulator: randoms[6],
        };

        let addr = (regs.h as u16) << 8 | (regs.l as u16);
        cpu.set_memory_at(addr, randoms[1]);
        cpu.set_all_registers(regs);

        for i in 0..=6
        {
            cpu.clock();
            assert_eq!(cpu.get_memory_at(addr), randoms[i]);
        }
    }

}

