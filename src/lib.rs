
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

struct VM {
    ip: usize,
    code: Vec<usize>,
}

impl VM {
    pub fn new() -> VM {
        VM{ip: 0, code: vec![0; 10]}
    }

    fn opcode_0(self) { println!("r > executing opcode 0") }
    fn opcode_1(self) { println!("r > executing opcode 1") }
    
    pub fn step(mut self) {
        self.ip += 1;
        match self.code[self.ip] {
            0 => self.opcode_0(),
            1 => self.opcode_1(),
            _ => {},
        }
    }
}

// impl IntoPy<PyObject> for VM {
//     fn into_py(self, py: Python<'_>) -> PyObject {
//         self.0
//     }
// }

fn run_python() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("r > Hello {}, I'm Python {}", user, version);

        let code = "vm.method_m(2)";
        let rv: String = py.eval(code, None, Some(&locals))?.extract()?;
        println!("r > rv: {}", rv);

        Ok(())
    })
}

// #[pyfunction]
// fn vm_init() -> PyResult<VM> {
//     println!("r > vm_init");
//     let vm = VM::new();
//     Ok(vm)
// }

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    println!("r > sum_as_string");
    run_python();
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyox(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // m.add_function(wrap_pyfunction!(vm_init, m)?)?;
    Ok(())
}