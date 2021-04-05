#[cfg(test)]
mod ret_test {
    use crate::gameboy::{Gameboy, RegisterLabel16, TickResult};

    #[test]
    fn calling_tick_up_to_a_breakpoint_will_cause_the_gb_to_stop() {
        let mut gb = Gameboy::new(vec![]); // Gameboy full of nop's

        let dt = 1.0 / 60.0;
        let breakpoints = vec![0x05];
        let stop_reason = gb.tick_with_breaks(dt, &breakpoints);

        assert_eq!(stop_reason, TickResult::HitBreakpoint);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x05);
    }

    #[test]
    fn calling_tick_on_a_loop_will_cause_the_frame_to_complete() {
        let mut gb = Gameboy::new(vec![0x0, 0x18, 0xFD]); // NOP, JR -3

        println!("{:?}", gb.get_opcode_with_offset(0x00).unwrap());
        println!("{:?}", gb.get_opcode_with_offset(0x01).unwrap());

        let dt = 1.0 / 60.0;
        let breakpoints = vec![0x05];
        let stop_reason = gb.tick_with_breaks(dt, &breakpoints);

        assert_eq!(stop_reason, TickResult::FrameComplete);
        assert!(gb.get_register_16(RegisterLabel16::ProgramCounter) < 0x02);
    }

    #[test]
    fn ticking_from_a_breakpoint_will_continue_to_the_next_breakpoint() {
        let mut gb = Gameboy::new(vec![]);

        let dt = 1.0 / 60.0;
        let breakpoints = vec![0x01, 0x03];

        gb.tick_with_breaks(dt, &breakpoints);
        gb.tick_with_breaks(dt, &breakpoints);

        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x03);
    }
}
