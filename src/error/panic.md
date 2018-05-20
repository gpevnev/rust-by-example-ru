# `panic`

Самый простой механизм обработки ошибок, с которым мы познакомимся – это `panic`.
Он печатает сообщение с ошибкой, начинает процедуру
раскрутки стека и, чаще всего, завершает программу.

В данном примере мы явно вызывает `panic` в случае ошибки:

```rust,editable,ignore,mdbook-runnable
fn give_princess(gift: &str) {
    // Принцесса ненавидит змей, поэтому нам нужно остановиться, если она не одобрит!
    if gift == "змея" { panic!("AAAaaaaa!!!!"); }

    println!("Я люблю тебя, {}!!!!!", gift);
}

fn main() {
    give_princess("плюшевый мишка");
    give_princess("змея");
}
```