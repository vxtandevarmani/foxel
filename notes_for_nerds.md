/* Hey this is my first time writing rust uhh please give me any suggestions lol */


/**
 * bits_to_opcode() =>  A helper function used to convert 
 *                      opcodes into their enum represenation
 *                      for match statements in a switch-loop
 *                      based dispatch subroutine however, a more
 *                      efficient method of dispatch was used for
 *                      performance [ref1] enhancements and making
 *                      reverse engineering a tad bit tedious
 * 
 * [ref1]
 * ==>  So the performance increase is due to leveraging the cpu pipeline
 *      to handle instructions since this method technically "unrolls" the
 *      loop making Out-Of-Order Execution (OoO) more viable and also removes
 *      conditional jumps which hinders Branch Prediction since the predictor
 *      cannot reliably predict which path to take and if it guesses wrong it
 *      is forced to flush the Branch Target Buffer (BTB) also the compiler doesnt
 *      add in any other bounds of checking
 */


/**
 *     --> also bottom 2 bits of the riscv opcodes are always 
 *      set so you can define a handler table 32 elements
 *      long and add a (>> 2) in every fetch cool optimization trick :p 
 *      You can see this behavior in helpers.rs:build_table() and in
 *      macros.rs:dispatch!()
 */


/**
 *  [How I got Tail Call Optimizations (TCOs) on Rust]
 *  
 *  Everything here is located within vm.rs
 *
 *  ->  Why TCOs?
 *      #>  Well first theyre a method in utilizing direct threading
 *          and another thing is to allow the VM be more architecture
 *          compatible since LLVM is handling the generation rather than
 *          us inserting asm blocks which severely limit the portability
 *          of the vm
 *
 *  -> How?
 *      #>  Okay so unlike C/C++/Zig/etc languages Rust doesnt really allow
 *          us to force TCOs so we basically have to beg and pray that the
 *          compiler inserts this for us and theres multiple ways we can do
 *          this.
 *              1.  Make sure that optimizations are on such as opt-level,
 *                  code-gen, flto those are the ones that from my experience
 *                  lead to the generation of TCOs.
 *
 *              2.  So between the Caller (dispatcher) and the Callee (handler)
 *                  there has to be only 1 clean stack frame which is why the
 *                  #[inline(never)] is put in order to ensure that a full function
 *                  is emitted since TCOs utilize the current frame rather than a
 *                  new frame.
 *              
 *              3.  The function signature of the Caller (dispatcher) and the
 *                  Callee (handler) must match hence the extern "C" since according
 *                  to the "internet" Rust doesnt always use the C standard calling
 *                  convention so the extern "C" essentially forces that and this is why
 *                  we have a run() and a thread() function since thread() and the other
 *                  handler functions have the same function signature as it is one of the
 *                  requirements of LLVM code generation according to the "internet" since
 *                  you dont have to re-set the arguments again if theyre already in the same
 *                  place before you jump
 */


/**
 * [How do I manage vm exits compared to secret club]
 *      
 *              (Everything here is located in vm.rs/new())
 *
 *      #>  Okay so the way that Secret club handles exits is by
 *          inserting an exit() syscall stub and a ebreak instruction
 *          after the exit in the payload before compiling which can be seen
 *          here in riscvm/lib/crt0.c[8-26]
 *
 *          ```
 *          void _start()
 *          {
 *              riscvm_relocs();
 *              riscvm_imports();
 *              riscvm_init_arrays();
 *              exit(main());
 *              asm volatile("ebreak");
 *          }
 * 
 *          #include <stdint.h>
 *          #include <stddef.h>
 *          #include <stdbool.h>
 *          static __attribute((noinline)) void exit(int exit_code)
 *          {
 *              register uintptr_t a0 asm("a0") = exit_code;
 *              register uintptr_t a7 asm("a7") = 10000;
 *              asm volatile("scall" : "+r"(a0) : "r"(a7) : "memory");
 *          }
 *          ```
 * 
 *          and the instructions to handle these are located at riscvm/riscvm.cpp[160-163]
 *          
 *          ```
 *          case 10000: // exit
 *          {
 *              return false;
 *          }
 *          ```
 * 
 *          and here in riscvm/riscvm.cpp[1019-1023]
 *          
 *          ```
 *          case 0b000000000001: // ebreak
 *          {
 *              reg_write(reg_a0, -1);
 *              return false;
 *          }
 *          ```
 *      
 *          which both show how secret club handles the way their vm exits from the payload code to the vm.
 *          
 *          The way I do it is rather different since instead of relying on the payload to exit I have taken
 *          a more of a subroutine based approach which is allocating an ebreak instruction on top of the stack
 *          during initialization so at the end of program flow the 
 *          ```
 *          jalr x0, 0(x1)
 *          ```
 *          instruction will pop the final value on the stack and terminate upon the ebreak instruction.
 *          This approach does however, require that the compiler emit the following instructions 
 * 
 *          ```
 *          0: f8010113      addi    sp, sp, -0x80
 *          4: 06113c23      sd      ra, 0x78(sp)
 *          ```
 *          and end with the following instruction
 *          ```
 *          jalr x0, 0(x1)
 *          ```
 *          
 *          both of these behaviors are generated in various builds
 *          in writing the shellcode template so no worries for now
 *          
 * 
 *          Now if you wanna handwrite riscv assembly and assemble you can just write as you wish
 *          and insert an ebreak at the end to ensure that the program terminates as is
 * 
 *          
 *          #>      So why this? 
 *              --> First of all, this makes the implant a tad bit smaller and also allows individual
 *                  functions to be called like function pointers which allows coroutine executions
 *                  and allows an ease of integration when writing a native vm bridge
 *
 *              --> Second of all, I dont have to worry about various linker stuff and just makes development
 *                  a whole lot easier, less things that can go wrong (because if anything can go wrong in development
 *                  it will) and again im lazy (also im writing this like 3 months later so take everything with a grain of salt)
 *                  but again this and also the build system is a couple lines shorter
 *
 *              --> Third of all, idk lol Im writing this 3-4 months later I think its 
 *                  something about no relocs because I CBA and yea
 *                  whoops sorry I forgot :p
 *
 */

    -#> Why did you run away for 3 months?
        ->  uh I wanted to see if I wrote managable code 
            and the only way to test that is to forget
            about it then come back >,<