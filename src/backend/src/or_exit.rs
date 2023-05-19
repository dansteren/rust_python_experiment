use std::process;

use rustpython_vm::{
    builtins::PyBaseException, convert::ToPyObject, PyObjectRef, PyRef, VirtualMachine,
};

pub trait OrExit<T> {
    fn or_exit(self, vm: &VirtualMachine) -> T;
}

impl OrExit<PyObjectRef> for Result<PyObjectRef, PyRef<PyBaseException>> {
    fn or_exit(self, vm: &VirtualMachine) -> PyObjectRef {
        match self {
            Ok(ok) => return ok,
            Err(err) => {
                let type_name = err.clone().to_pyobject(vm).class().name().to_string();
                match &err.to_pyobject(vm).str(vm) {
                    Ok(string) => println!("{type_name}: {string}"),
                    Err(_) => {
                        println!("Attribute Error: '{type_name}' object has no attribute '__str__'")
                    }
                };
                process::exit(1)
            }
        }
    }
}
