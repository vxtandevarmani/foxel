use libvm::{
    riscvm::{
        vm::VmState,
    },
};

use misc::utils::*;

#[derive(Default)]
#[repr(C)]
pub struct Argument {
    pub name : *mut u8,
    pub length: u32,
}

fn main() {
    
    let mut keys = KeyWrapper {
        round_key: [21u8; 4 * 4 * 11],
        key:       [42u8; 4 * 4     ],
        nonce:     [69u8; 4 * 4     ],
    };

    let mut implant: Vec<u8> = include_bytes!("INSERT_PATH_HERE").to_vec();

    code_cryption(&mut implant, &mut keys);

    let mut vimp: VmState = VmState::new(&mut implant, Some(keys), 0x100usize);

    unsafe {
        let mut buf = vec![0u8; 17];
        let mut ll: Argument = Argument { name: buf.as_mut_ptr(), length: (17) };

        let inp = &mut ll as *mut Argument;

        vimp.run(inp as *const ());

        println!("{:}", String::from_utf8(buf).unwrap());
    }


    eprintln!("this is the return value {:?}", vimp.return_value());
    println!(
        "is the address in the virtual space: {:#?}", vimp.is_virtual_fn(implant.as_ptr().wrapping_add(10) as u64)
    );

}

/* okay stuff to do
    5. callback support
        --> context_capture()
        --> context_restore()
    6. macrofusion                      # maybe later
    7. opcode shuffling                 # maybe later
    8. other optimization bullshit      # maybe later (ill probably look into this if its viable)
    9. clean yo code and write the meson build
    10. turn this into a library

    YEA SCREWW THIS IM DOING THIS ALL LATER I GOT SHIT TO DO

*/