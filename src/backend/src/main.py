# from kybra import (CandidError)

try:
    a = ic.my_native_rust_method()
    print("The return value of ic.my_native_rust_method is:", a)
except SystemError as err:
    print("Calling my_native_rust_method() went wrong:", err)
except TypeError as err:
    print("There was a TypeError:", err)
except CandidError as err:
    print("There was a CandidError:", err)
except Exception as err:
    print("GENERAL exception:", err)
