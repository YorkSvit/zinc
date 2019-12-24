extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Lt;

impl<E, O> VMInstruction<E, O> for Lt
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let lt = vm.get_operator().lt(left, right)?;

        vm.push(Cell::Value(lt))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_lt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Lt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Lt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Lt)
            .test(&[1, 0, 0])
    }
}