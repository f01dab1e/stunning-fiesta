use crate::tables::AllocTable;

key![Infer];

pub enum InferData {
    Unbound(u32),
}

pub struct Unification {
    variables: AllocTable<Infer, InferData>,
}

impl Unification {
    fn mk_var(&mut self) -> Infer {
        self.variables.add(InferData::Unbound(0))
    }
}
