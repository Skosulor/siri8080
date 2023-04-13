mod tests

{
    use siri8080::i8080::instructions::{InstructionTypes, Instruction};
    use siri8080::i8080::registers::*;
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
}
