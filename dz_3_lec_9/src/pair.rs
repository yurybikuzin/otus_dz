// 1) Принимает мутабельную ссылку на кортеж и bool значение.
//      - Если false, возвращает мутабельную ссылку на первый элемент кортежа.
//      - Если true, возвращает мутабельную ссылку на второй элемент кортежа.
#[derive(Debug)]
pub struct Pair(f32, f32, f32);

pub fn mut_link_pair(pair: &mut Pair, a: bool) -> &mut f32 {
    if a {
        &mut pair.1 as &mut f32
    } else {
        &mut pair.0 as &mut f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_link_pair() {
        assert_eq!(mut_link_pair(&mut Pair(1.0, 3.0, 5.0), true), &mut 3.0);
        assert_eq!(mut_link_pair(&mut Pair(1.0, 3.0, 5.0), false), &mut 1.0);
    }
}
