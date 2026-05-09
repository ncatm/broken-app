pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов.
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let raw = Box::into_raw(boxed);

    unsafe {
        let slice = &*raw;
        let count = slice.iter().filter(|&&b| b != 0).count();
        drop(Box::from_raw(raw));
        count
    }
}

/// Нормализация строки: удаляем пробельные символы (Unicode) и приводим к нижнему регистру.
pub fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// Среднее арифметическое только по положительным элементам (пустой срез → 0.0).
pub fn average_positive(values: &[i64]) -> f64 {
    let mut sum = 0_i64;
    let mut count = 0_usize;
    for &v in values {
        if v > 0 {
            sum += v;
            count += 1;
        }
    }
    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

/// Ранее демонстрировала use-after-free; теперь безопасно возвращает удвоенное значение из бокса.
pub fn use_after_free() -> i32 {
    let val = *Box::new(42_i32);
    val * 2
}
