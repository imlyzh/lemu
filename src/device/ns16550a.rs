use crate::abstract_machine::{Readable, Writeable};





pub struct Ns16550a {

}

impl Ns16550a {
    pub fn new() -> Ns16550a {
        Ns16550a {
        }
    }
}

impl Readable for Ns16550a {
    fn read_u8(&self, _addr: usize) -> Option<u8> {
        todo!()
    }
}

impl Writeable for Ns16550a {
    fn write_u8(&self, _addr: usize, _value: u8) -> Option<()> {
        todo!()
    }


}