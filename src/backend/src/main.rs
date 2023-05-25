use or_exit::OrExit;
use rustpython_derive::{pyclass, PyPayload};
use rustpython_vm::{
    builtins::PyTypeRef, class::PyClassImpl, Interpreter, PyResult, TryFromObject, VirtualMachine,
};

mod or_exit;

#[pyclass(module = false, name = "ic")]
#[derive(Debug, PyPayload)]
struct Ic {}
#[pyclass]
impl Ic {
    #[pymethod]
    fn my_native_rust_method(&self, vm: &VirtualMachine) -> PyResult {
        let candid_error_class = vm.run_block_expr(
            vm.new_scope_with_builtins(),
            r#"
                from kybra import CandidError

                CandidError
            "#,
        )?;

        let candid_error_class = PyTypeRef::try_from_object(vm, candid_error_class)?;

        Err(vm.new_exception_msg(
            candid_error_class,
            "something went really wrong".to_string(),
        ))
    }
}

fn main() {
    let python_source_code = std::fs::read_to_string("src/backend/src/main.py").unwrap();

    let interpreter = Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    });
    let scope = interpreter.enter(|vm| vm.new_scope_with_builtins());

    interpreter.enter(|vm| {
        Ic::make_class(&vm.ctx);
        vm.builtins
            .set_attr("ic", vm.new_pyobj(Ic {}), vm)
            .expect("should succeed");

        let python_return_value = vm
            .run_code_string(scope.clone(), &python_source_code, "".to_owned())
            .or_exit(vm);

        println!(
            "Called ic.accept_message and got back an OK value: {}",
            python_return_value.class().name().to_string()
        )
    });
}
