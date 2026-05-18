# Исправленные дефекты и оптимизации

## Баги (7)

| # | Модуль | Проблема | Исправление |
|---|--------|----------|-------------|
| 1 | `lib::sum_even` | off-by-one + `get_unchecked` → UB | Итератор, `0..len` |
| 2 | `lib::leak_buffer` | утечка (`Box::into_raw` без `from_raw`) | `iter().filter().count()` |
| 3 | `lib::normalize` | только пробелы, не `\t\n` | `split_whitespace()` |
| 4 | `lib::average_positive` | деление на все элементы | только `v > 0` |
| 5 | `lib::use_after_free` | use-after-free | функция удалена |
| 6 | `concurrency` | `static mut` + data race | `AtomicU64` |
| 7 | `algo` | O(2ⁿ) fib, O(n²) dedup | O(n) итерация + `HashSet` |

## Оптимизации

1. **Алгоритмическая:** `slow_fib` — линейный цикл вместо рекурсии; `slow_dedup` — `HashSet` вместо вложенного поиска и сортировки на каждой вставке.
2. **Микро:** `leak_buffer` — без лишних аллокаций бокса; `sum_even` — без `unsafe`, итераторы вместо индексов.

## Ускорение (ориентир)

| Функция | До | После | Коэффициент |
|---------|-----|-------|-------------|
| `slow_dedup` | ~7.4 ms | ~56 µs (criterion) | ~130× |
| `slow_fib(32)` | экспоненциально (минуты в bench) | ~11 ns (criterion, const) | >>1000× |
| `sum_even` | ~15 µs | ~6.3 µs | ~2× |

Подробные логи: `artifacts/baseline_*.txt`, `artifacts/criterion_after.txt`.

## Регрессионные тесты

- `tests/integration.rs` — тесты `regression_*`
- `tests/concurrency.rs` — потокобезопасность и сброс счётчика
