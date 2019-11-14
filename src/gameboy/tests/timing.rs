#[cfg(test)]
mod ret_test {
    use crate::gameboy::{Gameboy, RegisterLabel16, TickResult};
    use rust_catch::tests;

    tests! {
        test("Calling tick up to a breakpoint will cause the GB to stop") {
            let mut gb = Gameboy::new(vec![]); // Gameboy full of nop's

            let dt = 1.0/60.0;
            let breakpoints = vec![0x05];
            let stop_reason = gb.tick(dt, &breakpoints);

            assert_eq!(stop_reason, TickResult::HitBreakpoint);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x05);
        }

        test("Calling tick on a loop will cause the frame to complete") {
            let mut gb = Gameboy::new(vec![0x0, 0x18, 0xFD]); // NOP, JR -3

            println!("{:?}", gb.get_opcode_with_offset(0x00).unwrap());
            println!("{:?}", gb.get_opcode_with_offset(0x01).unwrap());

            let dt = 1.0/60.0;
            let breakpoints = vec![0x05];
            let stop_reason = gb.tick(dt, &breakpoints);

            assert_eq!(stop_reason, TickResult::FrameComplete);
            assert!(gb.get_register_16(RegisterLabel16::ProgramCounter) < 0x02);
        }

        test("ticking from a breakpoint will continue to the next breakpoint") {
            let mut gb = Gameboy::new(vec![]);

            let dt = 1.0 / 60.0;
            let breakpoints = vec![0x01, 0x03];

            gb.tick(dt, &breakpoints);
            gb.tick(dt, &breakpoints);

            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x03);
        }
    }
}
