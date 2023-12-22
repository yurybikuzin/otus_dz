fn main() {
    // 1. функция double_int32 принимает 32-х битное целое беззнаковое число и возвращает 32-х битное целое беззнаковое число, равное удвоенному входному.
    fn double_int32(a: u32) -> u32 {
        let b: u32 = a * 2;
        println!("double_int32, число b: {b}");
        b
    }
    double_int32(4);

    // 2. функция double_int64 принимает 32-х битное целое беззнаковое число и возвращает 64-х битное целое беззнаковое число, равное удвоенному входному.
    fn double_int64(a: u32) -> u64 {
        let b: u64 = (a * 2).into(); // into() - конвертация одного типа в другой
        println!("double_int64, число b: {b}");
        b
    }
    double_int64(4);

    // 3. функция double_float32 принимает 32-х битное число с плавающей точкой и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.
    fn double_float32(a: f32) -> f32 {
        let b: f32 = a * 2.0;
        println!("double_float32, число b: {b}");
        b
    }
    double_float32(4.5);

    // 4. функция double_float64 принимает 32-х битное число с плавающей точкой и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
    fn double_float64(a: f32) -> f64 {
        let b: f64 = (a * 2.0).into();
        println!("double_float64, число b: {b}");
        b
    }
    double_float64(4.5);

    // 5. функция int_plus_float_to_float принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
    fn int_plus_float_to_float(a: u32, b: f32) -> f64 {
        let a1: f64 = a.into();
        let b1: f64 = b.into();
        let c: f64 = a1 + b1;
        println!("int_plus_float_to_float, число c: {c}");
        c
    }
    int_plus_float_to_float(4, 4.5);

    // 6. функция int_plus_float_to_int принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное целое беззнаковое число, равное сумме входных.
    fn int_plus_float_to_int(a: u32, b: f32) -> u64 {
        let a1: f64 = a.into();
        let b1: f64 = b.into();
        let c: u64 = (a1 + b1).round() as u64;
        println!("int_plus_float_to_int, число c: {c}");
        c
    }
    int_plus_float_to_int(4, 4.4);

    // 7. функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже.
    fn tuple_sum(a: (u64, u64)) -> u64 {
        let b: u64 = a.0 + a.1;
        println!("tuple_sum, число b: {b}");
        b
    }
    tuple_sum((4, 5));

    // 8. функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме ччисел во входном массиве.
    fn array_sum(a: [u32; 3]) -> u32 {
        let mut b: u32 = 0;
        for item in a {
            b += item;
        }
        println!("array_sum, число b: {b}");
        b
    }
    array_sum([4, 4, 1]);

}