use custos::{libs::cpu::CPU, AsDev, Matrix, BaseOps, VecRead};

#[test]
fn test_matrix_read() {
    let device = CPU::new().select();

    let matrix = Matrix::from(( &device, (2, 3), [1.51, 6.123, 7., 5.21, 8.62, 4.765]));
    let read = matrix.read();
    assert_eq!(&read, &[1.51, 6.123, 7., 5.21, 8.62, 4.765]);
}

#[test]
fn test_no_device() {
    {
        let device = CPU::new().select();
        let a = Matrix::from(( &device, (2, 3), [1.51, 6.123, 7., 5.21, 8.62, 4.765]));
        let b = Matrix::from(( &device, (2, 3), [1.51, 6.123, 7., 5.21, 8.62, 4.765]));
    
        let c = device.add(a, b);
        println!("{:?}", device.read(c.data()));
    }
    
}
