#![no_std]

/////////////
//  NOTES  //
/////////////


// -> Opting out of std doesn't change the size of your program.
// Rust programs only include the code from std that they use. So if you don't use std, like the programs on this page,
// you don't strictly speaking need to opt out of std to make small programs.
// I choose to opt out because it means fewer surprises of the form "hey, my program suddenly grew by 12kiB, what gives?"

//Let's declare a decent-sized image buffer in Rust.
//For simplicity, we'll put it at a fixed location in memory using static.
//This isn't idiomatic Rust, but by opting out of std we have given up our ability to allocate memory

const WIDTH: usize = 600;
const HEIGHT: usize = 600;


//let's add some global state to the WebAssembly module to keep track of the frame number.
use core::sync::atomic::{AtomicU32, Ordering};

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be
    // called from JavaScript. If you maintain that condition,
    // then we know that the &mut we're about to produce is
    // unique, and therefore safe.
    render_frame_safe(&mut BUFFER)
}

// We split this out so that we can escape 'unsafe' as quickly
// as possible.
/*fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    for pixel in buffer.iter_mut() {
        // WebAssembly is little-endian, so the u32 contains
        // the pixel components in the reversed order 0xAA_BB_GG_RR.
        *pixel = 0xFF_FF_00_FF;
    }
}*/

// Replace the old render_frame_safe with this:
/*fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = (x ^ y) as u32 | 0xFF_00_00_00;
        }
    }
}*/


 //Why? Well, WebAssembly currently doesn't have a sin operation
 //(though one has been proposed, and may be added in the future).
 //This means the Rust std library has to include its own implementation of sin, and
 //a high-quality implementation of sin takes a fair amount of code.

extern {
    fn js_sin(x: f32) -> f32;
}

fn sin(x: f32) -> f32 {
    unsafe { js_sin(x) }
}

fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    // This line is new:
    //  we want to update the BUFFER and then advance the FRAME.
    //  AtomicU32 provides a handy fetch_add operation that can retrieve
    //  the current frame number, and advance the global counter
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    //A word of caution: calling sin for every pixel like I showed above takes a lot of processing power.
    //This is why I haven't embedded the WebAssembly program here as an example:
    //it would drain your battery while you're reading.
    //Just like on computers of yore, call sin ahead of time and generate a lookup table!
     for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // NOTE: you don't actually want to write the
            // function this way. See the note at the end
            // of this section.
            let v = sin(x as f32) * 255.
                  + sin(y as f32) * 255.;
            buffer[y * WIDTH + x] =
                f.wrapping_add(v as u32) | 0xFF_00_00_00;
        }
    }
}

#[no_mangle]
pub extern fn the_answer() -> u32 {
    42
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
