/// 2) Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn slice_2(slice: &mut [usize], n: usize) -> &mut usize {
    &mut slice[n]
}

/// 3) Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn slice_3(slice: &[usize], n: usize) -> &usize {
    &slice[slice.len() - 1 - n]
}

/// 4) Принимает слайс и число N. Возвращает два слайса с элементами:
///      - с нулевого по N-1;
///      - с N-го по последний;
pub fn slice_4(slice: &[usize], n: usize) -> (&[usize], &[usize]) {
    let (slice1, slice2) = slice.split_at(n);
    (slice1, slice2)
}

/// 5) Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn slice_service(slice: &[usize]) -> (&[usize], &[usize]) {
    let len = slice.len();
    let num_item = len / 2 as usize;

    let (slice1, slice2) = slice_4(&slice, num_item);
    (slice1, slice2)
}

pub fn slice_5(slice: &[usize]) -> [&[usize]; 4] {
    let mut res_arr: [&[usize]; 4] = [&[]; 4];

    let (slice1, slice2) = slice_service(&slice);

    (res_arr[0], res_arr[1]) = slice_service(&slice1);
    (res_arr[2], res_arr[3]) = slice_service(&slice2);

    println!("res_arr {:?}", res_arr);

    res_arr
}

// ==================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_2() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        assert_eq!(slice_2(&mut arr[7..12], 3), &mut 11);
        assert_eq!(slice_2(&mut arr[..3], 0), &mut 1);
    }

    #[test]
    fn test_slice_3() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        assert_eq!(slice_3(&arr[7..12], 2), &10);
        assert_eq!(slice_3(&arr[..], 6), &8);
    }

    #[test]
    fn test_slice_4() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let (slice1, slice2) = slice_4(&arr[..7], 2);
        assert_eq!(slice1, [1, 2]);
        assert_eq!(slice2, [3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_slice_5() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        assert_eq!(slice_5(&arr[..]), [&[[1, 2, 3], [4, 5, 6, 7], [8, 9, 10], [11, 12, 13, 14]]; 4]);
    }
}
