use crate::AddAwait;
use syn::{Item, ItemConst, ItemFn, ItemStatic};

impl AddAwait for Item {
    fn add_await(&mut self) {
        match self {
            Self::Static(i) => i.add_await(),
            Self::Fn(i) => i.add_await(),
            Self::Const(i) => i.add_await(),
            _ => (),
        }
    }
}

impl AddAwait for ItemFn {
    fn add_await(&mut self) {
        self.block.add_await()
    }
}

impl AddAwait for ItemConst {
    fn add_await(&mut self) {
        /* 
            async await isn't allowed in 
            const context
        */
    }
}

impl AddAwait for ItemStatic {
    fn add_await(&mut self) {
        /*
            async await isn't allowd in
            const context
        */
    }
}
