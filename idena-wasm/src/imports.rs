use crate::backend::Backend;
use crate::environment::Env;
use crate::memory::read_region;

pub fn set_storage<B: Backend>(env: &Env<B>, addr: u32, offset: i32) -> () {
    let key = match read_region(&env.memory(), addr, 32) {
        Ok(k) => k,
        Err(e) => {
            println!("{}", e);
            return ()
        }
    };

    println!("{:?}", key);

    env.backend.unwrap().set_storage(key, Vec::new());
}