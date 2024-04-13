#![cfg(test)]

use super::*;

#[test]
fn create_opcode() {
    let most_significant = 0xF0;
    let least_significant = 0x0F;
    let opcode = Opcode::merge_bytes(most_significant, least_significant);
    let expected_opcode = Opcode::from(0xF00F);
    assert_eq!(opcode, expected_opcode);
}
