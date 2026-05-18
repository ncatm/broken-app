# Артефакты reference-app

Эталонная реализация для сверки поведения `broken-app`.

| Файл | Описание |
|------|----------|
| `cargo_test.log` | `cargo test` |
| `miri.log` | `cargo +nightly miri test` |
| `valgrind.log` | Valgrind в Docker (`./scripts/valgrind-docker.sh reference-app`) |
| `baseline.txt` | `cargo bench --bench baseline` |
| `criterion.txt` | `cargo bench --bench criterion` |

Commit эталона: `38ebe81795e90196227585a1d1890d7952ba7889`.
