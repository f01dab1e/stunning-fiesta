use crate::tables::Tables;

pub trait Debug: Sized {
    fn debug_with<'data, 'tables>(
        &'data self,
        tables: &'tables Tables,
    ) -> DebugWith<'data, 'tables, Self> {
        DebugWith(self, tables)
    }

    fn fmt(&self, tables: &Tables, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

pub struct DebugWith<'data, 'tables, T>(&'data T, &'tables Tables);

impl<'data, 'arena, T: Debug> std::fmt::Debug for DebugWith<'data, 'arena, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.0, self.1, f)
    }
}
