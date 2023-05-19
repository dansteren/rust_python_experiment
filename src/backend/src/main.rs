use rustpython_derive::{pyclass, PyPayload};
use rustpython_vm::{
    class::PyClassImpl, convert::ToPyObject, Interpreter, PyObjectRef, VirtualMachine,
};

#[pyclass(module = false, name = "ic")]
#[derive(Debug, PyPayload)]
struct Ic {}
#[pyclass]
impl Ic {
    #[pymethod]
    fn my_native_rust_method(&self, vm: &VirtualMachine) -> PyObjectRef {
        println!("The ic.my_native_rust_method method was called");
        let rust_string = "RUSTY!".to_string();
        rust_string.to_pyobject(vm)
    }
}

fn main() {
    let interpreter = Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    });
    let scope = interpreter.enter(|vm| vm.new_scope_with_builtins());

    interpreter.enter(|vm| {
        Ic::make_class(&vm.ctx);
        vm.builtins.set_attr("ic", vm.new_pyobj(Ic {}), vm).expect("should succeed");

        let result = vm.run_code_string(
            scope.clone(),
            &format!("a = ic.my_native_rust_method()\nprint(\"The return value of ic.my_native_rust_method is:\", a)"),
            "".to_owned(),
        );

        match result {
            Ok(ok_value) => println!(
                "Called ic.accept_message and got back an OK value: {}",
                ok_value.class().name().to_string()
            ),
            Err(_err_value) => println!("Called ic.accept_message and got back an Err value"),
        }
    });
}
