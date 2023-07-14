use pyo3::prelude::*;
use fixed::types::I64F64;

//use rust_decimal::Decimal;

type Decimal = I64F64;

#[pyclass]
#[derive(Clone, Copy, Debug)]
struct Order {
    order_id: i32,
    price: Decimal,
    qty: Decimal,
}
#[pymethods]
impl Order {
    #[new]
    fn new(order_id: i32, price: Decimal, qty: Decimal) -> Self {
        Order {
            order_id,
            price,
            qty,
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(name = "sum_as_str")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
fn order_value(order: Order) -> Decimal {
    order.price * order.qty
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
#[pyo3(name = "pyo3_ex")]
fn pyo3_example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Order>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(order_value, m)?)?;
    Ok(())
}