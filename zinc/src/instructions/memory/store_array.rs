use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::StoreSequence;

impl<E, O> VMInstruction<E, O> for StoreSequence
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.pop()?;
            vm.store(self.address + i, value)?;
        }

        Ok(())
    }
}