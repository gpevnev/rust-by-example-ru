struct A; // Конкретный тип `A`.
struct S(A); // Конкретный тип `S`.
struct SGen<T>(T); // Обобщённый тип `SGen`.

// Все следующие функции становятся владельцем переменной, переданной в них.
// После передачи, она сразу выходит из области видимости и освобождается.

// Объявляем функцию `reg_fn`, которая принимает аргумент `_s` типа `S`.
// Здесь отсутствует `<T>`, поэтому это не обобщённая функция.
fn reg_fn(_s: S) {}

// Объявляем функцию `gen_spec_t`, которая принимает аргумент `_s` типа `SGen<T>`.
// В ней явно задан параметр типа `A`, но поскольку `A` не был указан
// как параметр обобщённого типа для `gen_spec_t`, то он не является обобщённым.
fn gen_spec_t(_s: SGen<A>) {}

// Объявляем функцию `gen_spec_i32`, которая принимает аргумент `_s` типа `SGen<i32>`.
// В ней явно задан тип `i32`, который является определённым типом.
// Поскольку `i32` не является обобщённым типом, эта функция
// также не является обобщённой.
fn gen_spec_i32(_s: SGen<i32>) {}

// Объявляем функцию `generic`, которая принимает аргумент `_s` типа `SGen<T>`.
// Поскольку `SGen<T>` предшествует `<T>`, эта функция
// является обобщённой над `T`.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Используем не обобщённые функции.
    reg_fn(S(A)); // Конкретный тип.
    gen_spec_t(SGen(A)); // Неявно определён тип параметра `A`.
    gen_spec_i32(SGen(6)); // Неявно определён тип параметра `i32`.

    // Явно определён тип параметра `char` в `generic()`.
    generic::<char>(SGen('a'));

    // Неявно определён параметр типа `char` в `generic()`.
    generic(SGen('c'));
}
