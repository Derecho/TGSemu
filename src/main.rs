// TGS emulator
// Made by Derecho

extern crate libc;
extern crate termios;
extern crate tgsemu;

use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::env;
use std::process;
use libc::funcs::posix88::unistd;
use termios::*;
use tgsemu::tgs;
use tgsemu::tgs::TGSUI;

struct TerminalUI;
impl tgs::TGSUI for TerminalUI {
    fn init(&self) {
        print!("\x1B[?25l");  // Hide cursor
        print!("\x1B[2J");  // Clear screen
        // Disable line-buffering in terminal and echoing of characters
        let fd = 0;
        let mut termios = Termios::from_fd(fd).unwrap();
        termios.c_lflag = termios.c_lflag & !ICANON;
        termios.c_lflag = termios.c_lflag & !ECHO;
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 0;
        tcsetattr(fd, TCSANOW, &termios).unwrap();
    }

    fn update_display(&self, dp: [u8; 4]) {
        //  _   _   _   _
        // |_| |_| |_| |_|
        // |_| |_| |_| |_| 
        //
        //  1
        // 6 2
        //  7
        // 5 3
        //  4

        print!("\x1B[0;0H");  // Reset cursor

        // Line 1
        print!("  ");
        if (dp[3] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (dp[2] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (dp[1] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (dp[0] & 1) == 1 {
            print!("_");
        }

        print!("\n");
        io::stdout().flush().ok();

        // Line 2
        print!(" ");
        if ((dp[3] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[2] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[1] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[0] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 1) & 1) == 1 {
            print!("|");
        }

        print!("\n");
        io::stdout().flush().ok();

        // Line 3
        print!(" ");
        if ((dp[3] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[2] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[1] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[0] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 2) & 1) == 1 {
            print!("|");
        }

        print!("\n");
        io::stdout().flush().ok();
    }

    fn update_buttons(&self, bt: &mut [u8; 2]) {
        let mut chr = [0; 1];
        if io::stdin().read(&mut chr).ok().unwrap() == 1 {
            if chr[0] == 97 {
                println!("A pressed!");
                bt[0] = 1;
                bt[1] = 0;
            }
            else if chr[0] == 98 {
                println!("B pressed!");
                bt[0] = 0;
                bt[1] = 1;
            }
        }
        else {
            bt[0] = 0;
            bt[1] = 0;
            println!("          ");
        }
    }
}

fn main() {
    // Check arguments
    if env::args().count() != 2 {
        println!("Usage: {} <rom.bin>", env::args().next().unwrap());
        process::exit(1);
    }

    // Read ROM file
    let filename = env::args().skip(1).next().unwrap();
    let mut file = File::open(filename).ok().expect("Error opening ROM file");
    let mut program: Vec<[u8; 3]> = Vec::new();
    let mut buf: [u8; 3] = [0; 3];
    loop {
        let n = file.read(&mut buf).ok().unwrap();
        if n == 0 {
            // Finished reading file
            break;
        }
        else if n != 3 {
            panic!("Invalid ROM file");
        }
        program.push(buf);
    }

    // Set up game console
    let ui = TerminalUI;
    ui.init();  // Prepare terminal
    let mut tgs = tgs::TGS::new(ui);

    // Main emulation loop
    let mut pc = 0;
    let mut cycles = 0;
    loop {
        pc = tgs.instruct(program[pc][0], program[pc][1], program[pc][2]) as usize;
        
        // Every 2000 cycles, or 25Hz, update IO
        if cycles == 2000 {
            cycles = 0;

            tgs.update_display();
            tgs.update_buttons();
        }

        // Sleep for 20uS to approximate 50kHz clock. Real clock speed will be less due to time
        // taken by all calls in this loop.
        unsafe {
            unistd::usleep(20);
        }

        cycles = cycles + 1;
    }
}
