use std::rc::Rc;

// this type is not Send or Sync
struct Struct {
    // adds 8 bytes to every instance
    _not_send_or_sync: Rc<()>,
}

fn main() {}
