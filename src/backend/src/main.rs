use indoc::indoc;
use or_exit::OrExit;
use rustpython_derive::{pyclass, PyPayload};
use rustpython_vm::{class::PyClassImpl, Interpreter, PyResult, VirtualMachine};

mod or_exit;

const PYTHON_SOURCE_CODE: &str = indoc! {r#"
    a = ic.my_native_rust_method()
    print("The return value of ic.my_native_rust_method is:", a)
"#};

#[pyclass(module = false, name = "ic")]
#[derive(Debug, PyPayload)]
struct Ic {}
#[pyclass]
impl Ic {
    #[pymethod]
    fn my_native_rust_method(&self, vm: &VirtualMachine) -> PyResult {
        println!("The ic.my_native_rust_method method was called");
        // Err(vm.new_value_error("this is an exception!".to_string()));
        Err(vm.new_system_error("something went really wrong".to_string()))
    }
}

fn main() {
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
            .run_code_string(scope.clone(), PYTHON_SOURCE_CODE, "".to_owned())
            .or_exit(vm);

        println!(
            "Called ic.accept_message and got back an OK value: {}",
            python_return_value.class().name().to_string()
        )
    });
}
