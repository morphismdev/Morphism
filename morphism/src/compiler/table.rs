use core::marker::PhantomData;

use crate::compiler::compiler_closed_table::ClosedTableGetAt;
use crate::{Key, OpOnce, OpTy, OpTyOut, Tagged};

/// Generic closed-table dispatcher: program AST -> result, via a handler table.
///
/// This is the shared implementation for both lowering and reification.
/// Table selection uses flat indexing (`ClosedTableGetAt<Ix>`) where `U11` means the 12th element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByTable<Input, Table> {
    table: Table,
    _input: PhantomData<fn() -> Input>,
}

impl<Input, Table> ByTable<Input, Table> {
    #[inline]
    pub const fn new(table: Table) -> Self {
        Self {
            table,
            _input: PhantomData,
        }
    }

    #[inline]
    pub fn into_table(self) -> Table {
        self.table
    }
}

/// Type-level dispatch via closed table.
impl<Input, D, R, Ix, Payload, Table, HandlerOp> OpTy<Tagged<Key<D, R, Ix>, Payload>>
    for ByTable<Input, Table>
where
    ClosedTableGetAt<Ix>: OpTy<Table, OutTy = HandlerOp>,
    HandlerOp: OpTy<Payload>,
{
    type OutTy = OpTyOut<HandlerOp, Payload>;
}

/// Value-level dispatch via closed table.
impl<Input, D, R, Ix, Payload, Table, HandlerOp> OpOnce<Tagged<Key<D, R, Ix>, Payload>>
    for ByTable<Input, Table>
where
    ClosedTableGetAt<Ix>: OpOnce<Table, OutVal = HandlerOp>,
    HandlerOp: OpOnce<Payload>,
{
    type OutVal = <HandlerOp as OpOnce<Payload>>::OutVal;

    #[inline]
    fn run(self, prog: Tagged<Key<D, R, Ix>, Payload>) -> Self::OutVal {
        let handler_op = ClosedTableGetAt::<Ix>::new().run(self.table);
        handler_op.run(prog.into_inner())
    }
}

/// Closed-table lowerer: program AST -> lowered program AST, via a handler table.
///
/// The selected table entry is a **lowering op**: it maps a payload to a (possibly different) lowered program.
///
/// Table selection uses flat indexing (`ClosedTableGetAt<Ix>`) where `U11` means the 12th element.
pub type LowerByTable<Input, Table> = ByTable<Input, Table>;

/// Closed-table reifier: program AST -> runnable op, via a handler table.
///
/// This is the **closed-world** reification mechanism: programs containing keys not in the
/// table will fail to type-check. The table is selected via `ClosedTableGetAt<Ix>` using
/// the key's index.
///
/// Mental model:
/// - a program is `Tagged<Key<Domain, Role, Ix>, Payload>`
/// - `Ix` selects an entry from a handler table
/// - the selected entry is a "compile op" that maps `Payload -> RunnableOp`
/// - the runnable op is returned
///
/// NOTE: Table selection uses `ClosedTableGetAt<Ix>` (flat semantics), which provides a
/// logical contiguous view over the chunked HList-tuple encoding.
/// `U11` means the 12th element overall (first element inside the tail), consistent with
/// registry indices where `U11` is a normal index.
pub type ReifyByTable<Input, Table> = ByTable<Input, Table>;
