// 2) Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn slice_2(slice: &mut [usize], n: usize) -> &mut usize {
    &mut slice[n]
}

// 3) Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn slice_3(slice: &[usize], n: usize) -> &usize {
    &slice[slice.len() - 1 - n]
}

// 4) Принимает слайс и число N. Возвращает два слайса с элементами:
//      - с нулевого по N-1;
//      - с N-го по последний;
pub fn slice_4(slice: &[usize], n: usize) -> (&[usize], &[usize]) {
    let slice1 = &slice[..n];
    let slice2 = &slice[n - 1..];
    // let (slice1, slice2) = slice.split_at(n);
    // вторая часть не включает "n"
    (slice1, slice2)
}

// 5) Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn slice_5(slice: &[usize]) {
    // [&[usize]; 4]

    let len = slice.len();
    let remainder = len % 4;

    println!("len {}", len);
    println!("remainder {}", remainder);

    let mut amount_elements = 0;

    if remainder > 0 {
        amount_elements = (len - remainder) / 4;
        println!("amount_elements с остатком {}", amount_elements);
    } else {
        amount_elements = len / 4;
        println!("amount_elements без остатка {}", amount_elements);
    }

    // всего частей может быть - 1, 4 или 5

    // let mut res_arr: [&[usize]; 4]; // &[['R', 'u'], ['s', 't']]
    // как объявить массив слайлов?

    let mut n = 0;
    for chunk in slice.chunks(amount_elements) {
        println!("chunk {:?}", chunk);

        if n == 5 {
            //
            println!(" ");
        } else {
            // res_arr[n] = chunk;
        }
        n += 1;
    }
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
        assert_eq!(slice2, [2, 3, 4, 5, 6, 7]);
    }

    // #[test]
    // fn test_slice_5() {
    //     let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    // }
}
