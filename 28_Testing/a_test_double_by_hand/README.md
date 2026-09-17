# A test double is a second `impl`

**Level:** 201 → 301 · deep dive

**One line:** To hand a function a fake dependency in a test, the function has to ask for a trait instead of a concrete type. After that, a fake is an ordinary struct with an ordinary `impl`, and a mocking crate such as `mockall` only writes that struct for you.

```rust
use std::cell::RefCell;

#[derive(Debug)]
struct MailError;

trait Mailer {
    fn send(&self, to: &str, body: &str) -> Result<(), MailError>;
}

fn remind(customers: &[&str], mailer: &impl Mailer) -> usize {
    customers.iter().filter(|c| mailer.send(c, "your invoice is overdue").is_ok()).count()
}

#[derive(Default)]
struct RecordingMailer {
    sent: RefCell<Vec<String>>,
}

impl Mailer for RecordingMailer {
    fn send(&self, to: &str, _body: &str) -> Result<(), MailError> {
        self.sent.borrow_mut().push(to.to_string());
        Ok(())
    }
}

#[test]
fn reminds_every_customer() {
    let mailer = RecordingMailer::default();
    assert_eq!(remind(&["acme", "initech"], &mailer), 2);
    assert_eq!(*mailer.sent.borrow(), ["acme", "initech"]);
}
```

Production code passes a real mail client that implements `Mailer`. The test passes a `RecordingMailer`. `remind` cannot tell the difference, and it never needs a mail server to be tested.

## First, refactor to an interface

Rust checks types at compile time, so a function declared as `fn remind(customers: &[&str], mailer: &SmtpClient)` accepts an `SmtpClient` and nothing else. There is no Python-style `mock.patch` that swaps it at run time, and no Mockito-style subclass that overrides its methods. To pass something else, the signature has to change: ask for *something that can send mail* rather than *this mail client*.

1. Write a trait with the methods the function actually calls, and no others. `Mailer` has one method because `remind` uses one.
2. Implement it for the real client.
3. Change the parameter to `&impl Mailer` (static dispatch, one compiled copy per type) or `&dyn Mailer` (dynamic dispatch, one copy and a vtable call). Either accepts every double on this page. [Static and dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md) covers the trade-off; for this purpose the choice does not matter. The example below uses both, one per version of the code.

This step is the hard part. Everything that follows, `mockall` included, only saves typing once it is done. It also has a cost: a trait with exactly one production implementation is a layer that exists for the tests. Sometimes that is worth it. Sometimes the better test is the real thing, isolated: [a temporary directory](../../04_Files/temp_dirs_in_tests/README.md) instead of a fake filesystem, a local HTTP server instead of a fake client (see [Mocking a server](../../07_Clients/mocking_a_server/README.md)), a throwaway database instead of a fake repository.

## Three fakes, three jobs

| The fake | What it does | What it is for | In the example |
|---|---|---|---|
| **recording** | does the work trivially and remembers every call | asserting what reached the outside world | `RecordingMailer` |
| **failing on demand** | returns an error for chosen inputs | error paths a real server almost never produces when you want it to | `Unreachable("initech")` |
| **scripted** | returns the next reply from a list, and panics when the list runs out | pinning an exact sequence of calls | `ScriptedMailer` |

The literature has names for these (spy, stub, mock, fake) and does not agree on the borders between them. The column that matters is the third one.

`send` takes `&self`, because the real client does not need to change to send a message. A fake that records calls does need to change, so the record lives in a [`RefCell` ↗](https://doc.rust-lang.org/std/cell/struct.RefCell.html). That is [interior mutability](../../09_Advanced/interior_mutability/README.md), and it is the normal shape of a hand-written test double. A double shared across threads uses a `Mutex` instead.

## Outcome tests and interaction tests

The example runs the same four tests against two versions of `send_reminders`. Version 1 sends one email per overdue invoice. Version 2 is a refactor that sends one email per customer, listing every overdue invoice in it. For the customer the result is the same: every overdue invoice was mentioned to the right person, and nobody who owes nothing got an email.

| Test | Asserts | v1 | v2 |
|---|---|---|---|
| `every_overdue_invoice_was_mentioned` | what the customers received | pass | pass |
| `an_unreachable_customer_is_reported` | what the function returned when a send failed | pass | pass |
| `send_was_called_three_times` | the exact calls, in order | pass | **FAIL** |
| `scripted_second_send_fails` | a reply script recorded from v1's calls | pass | **FAIL** |

The last two failures are not bugs. The call-counting test pinned the implementation instead of the behaviour. The scripted one is worse: its second reply was an error meant for acme's second invoice, v2's second call went to initech, and the test now reports the wrong customer as unreachable.

**Assert on outcomes by default. Pin calls only when the calls are the requirement:** a retry policy that must try exactly three times, a rate limit, *never email a customer who owes nothing*. The Advanced testing course gives the same advice about `mockall`'s `times()`: expectations couple the test to the implementation, so set them where the interaction is the point and nowhere else.

## What `mockall` writes for you

[`mockall` ↗](https://docs.rs/mockall) generates the scripted kind from the trait definition. Checked against `mockall` 0.15.0:

```rust
#[cfg_attr(test, mockall::automock)]
pub trait Mailer {
    fn send(&self, to: &str, body: &str) -> Result<(), MailError>;
}

#[test]
fn reminds_acme_once() {
    let mut mailer = MockMailer::new();
    mailer.expect_send()
        .withf(|to, _body| to == "acme")   // only for calls that match
        .times(1)                          // exactly once
        .returning(|_, _| Ok(()));         // and reply with this
    assert_eq!(remind(&["acme"], &mailer), 1);
}
```

`cfg_attr(test, …)` means `MockMailer` exists only in test builds. What the generated mock does when a test gets it wrong, from a run against 0.15.0:

| The test | The panic |
|---|---|
| calls a method with no expectation set | `MockMailer::send("x", "y"): No matching expectation found` |
| `times(2)`, then calls it a third time | `MockMailer::send: Expectation(<anything>) called 3 times which is more than the expected 2` |
| `times(1)`, never calls it | `MockMailer::send: Expectation(<anything>) called 0 time(s) which is fewer than expected 1`, raised when the mock is dropped |
| `send`, then a second trait method `outbox_len`, both `in_sequence`, called in the wrong order | `MockMailer::outbox_len(): Method sequence violation` |

Three more things from the same run and the docs:

- **Expectations are tried in the order they were set.** `times(1)` returning `Ok`, then `times(1)` returning `Err`: the first call got `Ok`, the second `Err`. That is `ScriptedMailer`, generated.
- **`mailer.checkpoint()`** checks every expectation set so far and panics right there if one is unmet, instead of when the mock is dropped. It is for a test that uses the same mock for setup and for the call under test, so that expectations meant for setup cannot quietly satisfy the second half.
- **A trait from another crate cannot take `#[automock]`**, because an attribute macro only sees the item it is attached to, and that item's definition is in someone else's source. `mockall::mock!` takes the trait's signature copied into your own code instead.

## If you are coming from another language

- **Python.** `unittest.mock.patch("app.smtp_client")` swaps the dependency at run time without touching the function's signature, so Python code rarely needs the refactor this page opens with. That is the biggest difference, and it cuts both ways: Rust makes you write the seam, and then the seam is visible in the signature to every reader. `MagicMock` accepts any call with any arguments, which is how a Python test ends up passing against a method that was renamed months ago. `create_autospec` is the fix, and a Rust double has that property from the start, because it has to implement the trait. `mock.assert_called_once_with(...)` is `times(1)` with `withf`, and it has the same coupling problem.
- **ABAP.** ABAP's test double framework, `cl_abap_testdouble=>create( 'ZIF_MAILER' )`, creates doubles for **interfaces only**. So an ABAP developer who has used it has already done this page's refactor: pull the calls into an interface, have the real class implement it, and inject it through the constructor or a setter. `configure_call( … )->returning( … )` is `expect_send().returning(…)`, and its expectation checks are `times()`. The difference is where the check happens: a Rust double that does not implement the trait fully is a compile error, not a runtime surprise.
- **Java.** Mockito can mock a concrete class by generating a subclass, and newer versions can mock final classes too, so Java tests often skip the interface. Rust has no inheritance to borrow. The trait is the only seam, so `mockall` needs one, as `cl_abap_testdouble` does. `verify(mock, times(3)).send(...)` is the call-counting test above, with the same failure on refactor.

---

## The verified output

<!-- output:a_test_double_by_hand -->
*Verified output of [`a_test_double_by_hand.rs`](examples/a_test_double_by_hand.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The fake records what the code did
   send("acme", "invoice 101 is 12 days overdue")
   send("acme", "invoice 103 is 40 days overdue")
   send("initech", "invoice 104 is 3 days overdue")
   failed = []

2. A fake can fail on demand
   Unreachable("initech"): failed = ["initech"]

3. The same tests against version 1 and the batched version 2
   v1, one email per invoice
     every_overdue_invoice_was_mentioned  pass
     an_unreachable_customer_is_reported  pass
     send_was_called_three_times          pass
     scripted_second_send_fails           pass
   v2, one email per customer
     every_overdue_invoice_was_mentioned  pass
     an_unreachable_customer_is_reported  pass
     send_was_called_three_times          FAIL  assertion `left == right` failed; left: ["acme", "initech"]; right: ["acme", "acme", "initech"]
     scripted_second_send_fails           FAIL  assertion `left == right` failed; left: ["initech"]; right: ["acme"]
   Both versions tell every customer about every overdue invoice.
   The outcome tests say so. The two tests that pinned the calls
   themselves fail on a refactor that broke nothing a customer sees.
```
<!-- /output -->

## Practice

**The clock you cannot wait for.** A coupon is valid for 7 days after `issued_at` (a Unix timestamp in seconds). `is_valid` currently calls `SystemTime::now()` inside. Testing the boundary would mean waiting a week, or a test that passes or fails depending on when it runs.

Refactor it to take a `Clock` trait. Write a fixed clock for tests, and test the second before the boundary, the boundary itself, and the second after. Decide which of the three is still valid and say why the test is the place that decision is written down. Keep the real clock working in `main`.

<details markdown="1">
<summary><strong>Solution</strong></summary>

<!-- source:a_test_double_by_hand_kata -->
*[`a_test_double_by_hand_kata.rs`](examples/a_test_double_by_hand_kata.rs) in full — pasted here by `tools/run_examples.py` from the file CI compiles and runs.*

```rust
//! Kata solution: the clock you cannot wait for.
//!
//!   rustc --edition 2024 a_test_double_by_hand_kata.rs -o /tmp/atdbhk && /tmp/atdbhk
//!   rustc --edition 2024 --test a_test_double_by_hand_kata.rs -o /tmp/atdbhkt && /tmp/atdbhkt

use std::time::{SystemTime, UNIX_EPOCH};

const WEEK: u64 = 7 * 24 * 60 * 60;

/// The one thing the coupon code needs to know about time.
trait Clock {
    fn now(&self) -> u64; // seconds since the Unix epoch
}

/// Production: asks the operating system.
struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("after 1970").as_secs()
    }
}

/// Tests: says whatever it was told to.
struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

/// Valid for 7 days after issue. The boundary second itself is the last valid
/// one: "valid for 7 days" includes the 7-day mark, and this is where that
/// reading is decided.
fn is_valid(issued_at: u64, clock: &impl Clock) -> bool {
    clock.now().saturating_sub(issued_at) <= WEEK
}

const ISSUED: u64 = 1_788_000_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_one_second_before_the_week_is_up() {
        assert!(is_valid(ISSUED, &FixedClock(ISSUED + WEEK - 1)));
    }

    #[test]
    fn valid_on_the_last_second() {
        assert!(is_valid(ISSUED, &FixedClock(ISSUED + WEEK)));
    }

    #[test]
    fn expired_one_second_later() {
        assert!(!is_valid(ISSUED, &FixedClock(ISSUED + WEEK + 1)));
    }

    #[test]
    fn a_clock_behind_the_issue_time_does_not_underflow() {
        assert!(is_valid(ISSUED, &FixedClock(ISSUED - 60)));
    }
}

fn main() {
    println!("1. The boundary, tested without waiting a week");
    for (label, offset) in [("issued + 7 days - 1s", -1i64), ("issued + 7 days     ", 0), ("issued + 7 days + 1s", 1)] {
        let now = (ISSUED + WEEK).checked_add_signed(offset).expect("in range");
        println!("   {label}  is_valid = {}", is_valid(ISSUED, &FixedClock(now)));
    }

    println!();
    println!("2. A clock that runs behind the issue time");
    println!("   issued - 60s          is_valid = {}", is_valid(ISSUED, &FixedClock(ISSUED - 60)));
    println!("   now - issued would underflow a u64; saturating_sub makes it 0.");
    println!("   Only a fixed clock can put now before the issue time on purpose.");

    println!();
    println!("3. The real clock still works in production code");
    let issued_long_ago = 1_000_000_000; // September 2001
    println!("   issued September 2001, checked on SystemClock: is_valid = {}", is_valid(issued_long_ago, &SystemClock));
    println!("   SystemClock and FixedClock are both `impl Clock`; is_valid never knows which.");

    println!();
    println!("4. Why the test is where the boundary is decided");
    println!("   \"Valid for 7 days\" does not say whether the 7-day second counts.");
    println!("   The code picks one with <= or <. valid_on_the_last_second is the");
    println!("   line that says which, and it fails if someone changes their mind silently.");
}
```
<!-- /source -->

<!-- output:a_test_double_by_hand_kata -->
*Verified output of [`a_test_double_by_hand_kata.rs`](examples/a_test_double_by_hand_kata.rs) — regenerated by `tools/run_examples.py`, never hand-typed.*

```text
1. The boundary, tested without waiting a week
   issued + 7 days - 1s  is_valid = true
   issued + 7 days       is_valid = true
   issued + 7 days + 1s  is_valid = false

2. A clock that runs behind the issue time
   issued - 60s          is_valid = true
   now - issued would underflow a u64; saturating_sub makes it 0.
   Only a fixed clock can put now before the issue time on purpose.

3. The real clock still works in production code
   issued September 2001, checked on SystemClock: is_valid = false
   SystemClock and FixedClock are both `impl Clock`; is_valid never knows which.

4. Why the test is where the boundary is decided
   "Valid for 7 days" does not say whether the 7-day second counts.
   The code picks one with <= or <. valid_on_the_last_second is the
   line that says which, and it fails if someone changes their mind silently.
```
<!-- /output -->

</details>

## See also

- [Static and dynamic dispatch](../../12_Traits/static_vs_dynamic_dispatch/README.md): `&impl Mailer` against `&dyn Mailer`
- [What a trait is](../../12_Traits/what_a_trait_is/README.md): the interface this page refactors to
- [What a test asserts](../what_a_test_asserts/README.md): the assertion that cannot fail, which a fake makes easy to write
- [Temporary directories in tests](../../04_Files/temp_dirs_in_tests/README.md): the real filesystem, isolated, instead of a fake one
- [Mocking a server](../../07_Clients/mocking_a_server/README.md): the double one level further out, at the HTTP boundary
- [Testing: courses and links](../resources/README.md): the Advanced Rust testing course's mocking section, which this page follows

## Po polsku

Po polsku mówi się *atrapa* albo *dubler testowy*, ale szuka się i tak słowem `mock`. W Ruście podmiana zależności w teście nie jest sztuczką czasu wykonania, jak `mock.patch` w Pythonie ani podklasa generowana przez Mockito, tylko zmianą sygnatury: funkcja, która przyjmuje konkretny typ `&SmtpClient`, nie przyjmie niczego innego. Trzeba ją **przepisać na interfejs**: wydzielić cechę (*trait*) z metodami, których funkcja naprawdę używa, zaimplementować ją dla prawdziwego klienta i przyjmować `&impl Mailer` albo `&dyn Mailer`. Dopiero wtedy atrapa staje się zwykłą strukturą ze zwykłym `impl`, a `mockall` jedynie pisze tę strukturę za nas. Kto korzystał w ABAP-ie z `cl_abap_testdouble`, zna ten krok, bo tamten framework też tworzy atrapy wyłącznie dla interfejsów.

Druga lekcja dotyczy tego, **co** test sprawdza. Test wyniku (każdy klient dostał informację o każdej zaległej fakturze) przeżywa refaktoryzację. Test interakcji (metoda `send` wywołana dokładnie trzy razy, w tej kolejności) pada na zmianie, która niczego klientom nie zepsuła; w przykładzie wystarczyło wysyłać jeden list na klienta zamiast jednego na fakturę. Skrypt odpowiedzi nagrany z pierwszej wersji zrobił coś gorszego: zgłosił jako nieosiągalnego niewłaściwego klienta. Dlatego `times()` w `mockall` stosuje się tam, gdzie liczba wywołań jest wymaganiem (ponawianie, limity), a nie wszędzie, gdzie się da.

**Szukaj po polsku:** mockowanie w Ruście · dubler testowy · wstrzykiwanie zależności · `rust mockall automock` · `rust trait test double` · `rust dependency injection testing`
