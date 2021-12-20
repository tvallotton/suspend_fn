use super::AddAwait;
use syn::Block;

impl AddAwait for Block {
    fn add_await(&mut self) {
        for stmt in &mut self.stmts {
            stmt.add_await()
        }
    }
}
