def generate_cargo_toml(canister_name: str) -> str:
    return f"""
[package]
name = "{canister_name}"
version = "0.0.0"
edition = "2018"

[profile.release]
opt-level = 'z'
codegen-units = 1

[lib]
crate-type = ["cdylib"]

[dependencies]
ic-cdk = "0.8.0-beta"
ic-cdk-macros = "0.6.10"
ic-cdk-timers = "0.1.2"
candid = {{ version = "0.9.0-beta.2", features = ["parser"] }}
kybra-vm-value-derive = {{ path = "./kybra_vm_value_derive" }}
# TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
# rustpython = {{ git = "https://github.com/demergent-labs/RustPython", rev = "7483ff7959103d7c4b2a15d718d4f7ad9b57b459", default-features = false, features = ["stdlib", "freeze-stdlib"] }}
rustpython = {{ git = "https://github.com/demergent-labs/RustPython", rev = "7483ff7959103d7c4b2a15d718d4f7ad9b57b459", default-features = false, features = ["stdlib"] }}
rustpython-vm = {{ git = "https://github.com/demergent-labs/RustPython", rev = "7483ff7959103d7c4b2a15d718d4f7ad9b57b459", default-features = false }}
rustpython-stdlib = {{ git = "https://github.com/demergent-labs/RustPython", rev = "7483ff7959103d7c4b2a15d718d4f7ad9b57b459", default-features = false, features = [] }}
rustpython-derive = {{ git = "https://github.com/demergent-labs/RustPython", rev = "7483ff7959103d7c4b2a15d718d4f7ad9b57b459", default-features = false, features = [] }}
# rustpython = {{ path = "/home/RustPython", default-features = false, features = ["stdlib"] }}
# rustpython-vm = {{ path = "/home/RustPython/vm", default-features = false }}
# rustpython-stdlib = {{ path = "/home/RustPython/stdlib", default-features = false, features = [] }}
# rustpython-derive = {{ path = "/home/RustPython/derive", default-features = false, features = [] }}
# TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
# rustpython-pylib = {{ git = "https://github.com/demergent-labs/RustPython", rev = "7483ff7959103d7c4b2a15d718d4f7ad9b57b459", default-features = false, features = [] }}
# rustpython = {{ path = "../../../../../../RustPython", default-features = false, features = [] }}
getrandom = {{ version = "0.2.3", features = ["custom"] }}
serde = "1.0.137"
async-recursion = "1.0.0"
ic-stable-structures = "0.5.2"
slotmap = "1.0.6"
rand = "0.8.5"
    """
