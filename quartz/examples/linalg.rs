fn main() {
    // example matrix usage:
    let mut mat = quartz::linalg::Matrix::identity(5);
    mat[(4, 0)] = 1337.13;
    mat.print();
}
