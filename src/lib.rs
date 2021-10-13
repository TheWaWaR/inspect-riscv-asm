#![no_std]
#![feature(asm)]

#[no_mangle]
pub fn ctrl_ifelse(a: u32) -> u32 {
    let b = if a > 3 {
        333
    } else if a == 3 {
        33
    } else {
        3
    };
    b
}

#[no_mangle]
pub fn ctrl_ifelse_asm(mut a: u32) -> u32 {
    /*
    sext.w  a1, a0
    addi    a3, zero, 3
    addi    a2, zero, 33
    bne     a1, a3, .LBB0_3
    addi    a0, zero, 333
    bgeu    a3, a1, .LBB0_4
    .LBB0_2:
    ret
    .LBB0_3:
    addi    a2, zero, 3
    addi    a0, zero, 333
    bltu    a3, a1, .LBB0_2
    .LBB0_4:
    mv      a0, a2
    ret
    */
    unsafe {
        asm!(
            "sext.w  a1, a0",
            "addi    a3, zero, 3",
            "addi    a2, zero, 33",
            "bne     a1, a3, 3f",
            "addi    a0, zero, 333",
            "bgeu    a3, a1, 4f",
            "2:",
            "ret",
            "3:" ,
            "addi    a2, zero, 3",
            "addi    a0, zero, 333",
            "bltu    a3, a1, 2b",
            "4:",
            "mv      a0, a2",
            inout("a0") a,
        )
    }
    a
}

#[no_mangle]
pub fn ctrl_loop(mut a: u32) -> u32 {
    loop {
        a += 2;
        if a >= 10 {
            break;
        }
    }
    a
}

#[no_mangle]
pub fn ctrl_loop_asm(mut a: u32) -> u32 {
    /*
    addiw   a1, a0, 2
    addi    a2, zero, 10
    bltu    a2, a1, .LBB2_2
    addi    a1, zero, 10
    .LBB2_2:
    not     a2, a0
    add     a1, a1, a2
    andi    a1, a1, -2
    addw    a0, a0, a1
    addi    a0, a0, 2
    ret
    */
    unsafe {
        asm!(
            "addiw   a1, a0, 2",
            "addi    a2, zero, 10",
            "bltu    a2, a1, 1f",
            "addi    a1, zero, 10",
            "1:",
            "not     a2, a0",
            "add     a1, a1, a2",
            "andi    a1, a1, -2",
            "addw    a0, a0, a1",
            "addi    a0, a0, 2",
            inout("a0") a,
        )
    }
    a
}
