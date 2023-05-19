# rust_python_experiment

A playground for testing out RustPython, especially trying to return Exceptions from a native rust function up to the Python layer.

The setup of this example is that we create an `ic` object in python that calls rust code. This object has just one method: `my_native_rust_method`. Calling this method from the python code will result in getting a native rust value (converted to a python value of course).

The main function instantiates the python vm, hooks up the ic object, and then runs this python code

```python
a = ic.my_native_rust_method()
print("The return value of ic.my_native_rust_method is:", a)
```

Running this example with `cargo run` results in the following output:

```plaintext
The ic.my_native_rust_method method was called
The return value of ic.my_native_rust_method is: RUSTY!
Called ic.accept_message and got back an OK value: NoneType
```

So, we are able to return a regular value from Rust to Python. The question is, can `my_native_rust_method` raise an exception?
