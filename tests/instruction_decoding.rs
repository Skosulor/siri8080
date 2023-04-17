mod tests

{
    use siri8080::i8080::instructions::{InstructionTypes, Instruction};
    use siri8080::i8080::registers::*;
    use siri8080::i8080::flags::*;

    #[test]
    fn set_get_flags()
    {
        let mut flag = StatusFlags::new();

        for i in 0..=0xFF
        {
            flag.set_flags_u8(i);
            assert_eq!(flag.get_flags_u8(), (i | 0x2) & 0b11010111);
        }

        assert_eq!(flag.carry_flag, true);
        assert_eq!(flag.auxiliary_flag, true);
        assert_eq!(flag.sign_flag, true);
        assert_eq!(flag.zero_flag, true);
        assert_eq!(flag.parity_flag, true);

    }

    #[test]
    fn decode_add_b()
    {
        let inst = Instruction::from_byte(0x80);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_add_c()
    {
        let inst = Instruction::from_byte(0x81);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_add_d()
    {
        let inst = Instruction::from_byte(0x82);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_add_e()
    {
        let inst = Instruction::from_byte(0x83);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_add_h()
    {
        let inst = Instruction::from_byte(0x84);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_add_l()
    {
        let inst = Instruction::from_byte(0x85);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_add_m()
    {
        let inst = Instruction::from_byte(0x86);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_adc_a()
    {
        let inst = Instruction::from_byte(0x87);
        assert_eq!(inst.inst_type, InstructionTypes::ADD);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

    #[test]
    fn decode_adc_b()
    {
        let inst = Instruction::from_byte(0x88);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_adc_c()
    {
        let inst = Instruction::from_byte(0x89);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_adc_d()
    {
        let inst = Instruction::from_byte(0x8A);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_adc_e()
    {
        let inst = Instruction::from_byte(0x8B);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_adc_h()
    {
        let inst = Instruction::from_byte(0x8C);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_adc_l()
    {
        let inst = Instruction::from_byte(0x8D);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_adc_m()
    {
        let inst = Instruction::from_byte(0x8E);
        assert_eq!(inst.inst_type, InstructionTypes::ADC);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_sub_b()
    {
        let inst = Instruction::from_byte(0x90);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_sub_c()
    {
        let inst = Instruction::from_byte(0x91);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_sub_d()
    {
        let inst = Instruction::from_byte(0x92);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_sub_e()
    {
        let inst = Instruction::from_byte(0x93);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_sub_h()
    {
        let inst = Instruction::from_byte(0x94);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_sub_l()
    {
        let inst = Instruction::from_byte(0x95);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_sub_m()
    {
        let inst = Instruction::from_byte(0x96);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_sub_a()
    {
        let inst = Instruction::from_byte(0x97);
        assert_eq!(inst.inst_type, InstructionTypes::SUB);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

    #[test]
    fn decode_sbb_b()
    {
        let inst = Instruction::from_byte(0x98);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_sbb_c()
    {
        let inst = Instruction::from_byte(0x99);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_sbb_d()
    {
        let inst = Instruction::from_byte(0x9A);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_sbb_e()
    {
        let inst = Instruction::from_byte(0x9B);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_sbb_h()
    {
        let inst = Instruction::from_byte(0x9C);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_sbb_l()
    {
        let inst = Instruction::from_byte(0x9D);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_sbb_m()
    {
        let inst = Instruction::from_byte(0x9E);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_sbb_a()
    {
        let inst = Instruction::from_byte(0x9F);
        assert_eq!(inst.inst_type, InstructionTypes::SBB);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

    #[test]
    fn decode_ana_b()
    {
        let inst = Instruction::from_byte(0xA0);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_ana_c()
    {
        let inst = Instruction::from_byte(0xA1);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_ana_d()
    {
        let inst = Instruction::from_byte(0xA2);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }
    
    #[test]
    fn decode_ana_e()
    {
        let inst = Instruction::from_byte(0xA3);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_ana_h()
    {
        let inst = Instruction::from_byte(0xA4);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_ana_l()
    {
        let inst = Instruction::from_byte(0xA5);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_ana_m()
    {
        let inst = Instruction::from_byte(0xA6);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_ana_a()
    {
        let inst = Instruction::from_byte(0xA7);
        assert_eq!(inst.inst_type, InstructionTypes::ANA);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

    #[test]
    fn decode_xra_b()
    {
        let inst = Instruction::from_byte(0xA8);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_xra_c()
    {
        let inst = Instruction::from_byte(0xA9);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_xra_d()
    {
        let inst = Instruction::from_byte(0xAA);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_xra_e()
    {
        let inst = Instruction::from_byte(0xAB);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_xra_h()
    {
        let inst = Instruction::from_byte(0xAC);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_xra_l()
    {
        let inst = Instruction::from_byte(0xAD);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_xra_m()
    {
        let inst = Instruction::from_byte(0xAE);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }


    #[test]
    fn decode_xra_a()
    {
        let inst = Instruction::from_byte(0xAF);
        assert_eq!(inst.inst_type, InstructionTypes::XRA);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

    #[test]
    fn decode_ora_b()
    {
        let inst = Instruction::from_byte(0xB0);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_ora_c()
    {
        let inst = Instruction::from_byte(0xB1);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_ora_d()
    {
        let inst = Instruction::from_byte(0xB2);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_ora_e()
    {
        let inst = Instruction::from_byte(0xB3);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }
    
    #[test]
    fn decode_ora_h()
    {
        let inst = Instruction::from_byte(0xB4);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_ora_l()
    {
        let inst = Instruction::from_byte(0xB5);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_ora_m()
    {
        let inst = Instruction::from_byte(0xB6);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_ora_a()
    {
        let inst = Instruction::from_byte(0xB7);
        assert_eq!(inst.inst_type, InstructionTypes::ORA);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

    #[test]
    fn decode_cmp_b()
    {
        let inst = Instruction::from_byte(0xB8);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), B_REG);
    }

    #[test]
    fn decode_cmp_c()
    {
        let inst = Instruction::from_byte(0xB9);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), C_REG);
    }

    #[test]
    fn decode_cmp_d()
    {
        let inst = Instruction::from_byte(0xBA);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), D_REG);
    }

    #[test]
    fn decode_cmp_e()
    {
        let inst = Instruction::from_byte(0xBB);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), E_REG);
    }

    #[test]
    fn decode_cmp_h()
    {
        let inst = Instruction::from_byte(0xBC);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), H_REG);
    }

    #[test]
    fn decode_cmp_l()
    {
        let inst = Instruction::from_byte(0xBD);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), L_REG);
    }

    #[test]
    fn decode_cmp_m()
    {
        let inst = Instruction::from_byte(0xBE);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), MEM_REF);
    }

    #[test]
    fn decode_cmp_a()
    {
        let inst = Instruction::from_byte(0xBF);
        assert_eq!(inst.inst_type, InstructionTypes::CMP);
        assert_eq!(inst.byte1.unwrap(), A_REG);
    }

}
