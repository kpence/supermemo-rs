fn pretty_print_code_at_address(address: u32, size: u32) {
    let mut code: Vec<u8> = Vec::with_capacity(size as usize);
    for i in 0..size {
        let byte: u8 = unsafe { std::ptr::read((address + i) as *const u8) };
        code.push(byte);
    }
    pretty_print_code(&code[..]);
}

fn pretty_print_code(code: &[u8]) {
    use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};
    let code: &[u8] = &code[..];
    const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
    const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;

    let mut decoder =
        Decoder::with_ip(32, code, 0x0000_7FFA_C46A_CDA4, DecoderOptions::NONE);
    let mut formatter = NasmFormatter::new();

    // Change some options, there are many more
    formatter.options_mut().set_digit_separator("`");
    formatter.options_mut().set_first_operand_char_index(10);

    // String implements FormatterOutput
    let mut output = String::new();

    // Initialize this outside the loop because decode_out() writes to every field
    let mut instruction = Instruction::default();

    // The decoder also implements Iterator/IntoIterator so you could use a for loop:
    //      for instruction in &mut decoder { /* ... */ }
    // or collect():
    //      let instructions: Vec<_> = decoder.into_iter().collect();
    // but can_decode()/decode_out() is a little faster:
    while decoder.can_decode() {
        // There's also a decode() method that returns an instruction but that also
        // means it copies an instruction (40 bytes):
        //     instruction = decoder.decode();
        decoder.decode_out(&mut instruction);

        // Format the instruction ("disassemble" it)
        output.clear();
        formatter.format(&instruction, &mut output);

        // Eg. "00007FFAC46ACDB2 488DAC2400FFFFFF     lea       rbp,[rsp-100h]"
        print!("{:016X} ", instruction.ip());
        let start_index = (instruction.ip() - EXAMPLE_CODE_RIP) as usize;
        let instr_bytes = &code[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02X}", b);
        }
        if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
            for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", output);
    }
}
