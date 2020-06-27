#[cfg(test)]
mod alu_test {
    use super::super::tests::infinite_loop_gb;
    use rust_catch::tests;

    tests! {
        test("Run the gameboy for a frame and get a frame of audio") {
            let mut gb = infinite_loop_gb();

            // Run the gameboy for a frame
            let breakpoints = vec![];
            gb.tick(1.0 / 60.0, &breakpoints);

            let sample = gb.get_sample();
            assert!(sample.iter().all(|val| *val == 0));

            assert_eq!(sample.len(), 800);
        }

        test("Run into the next frame and the buffer is cleared") {
            let mut gb = infinite_loop_gb();

            let breakpoints = vec![];
            gb.tick(1.0 / 60.0, &breakpoints);

            // This should put us onto the next frame
            for _ in 0..6 { // Run through at least 87 frames
                gb.step_once();
                gb.step_once();
            }

            let sample = gb.get_sample();
            assert_eq!(sample.len(), 1);
        }
    }
}
