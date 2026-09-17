//! A test double is a second `impl` of a trait the code under test asks for.
//!
//!   rustc --edition 2024 a_test_double_by_hand.rs -o /tmp/atdbh && /tmp/atdbh
//!
//! `main` runs each "test" through `check`, which catches the panic a failed
//! assertion raises, so both outcomes land in the recorded output.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::panic;

#[derive(Debug)]
struct MailError;

/// What the reminder code needs from a mail system, and nothing else.
trait Mailer {
    fn send(&self, to: &str, body: &str) -> Result<(), MailError>;
}

struct Invoice {
    customer: &'static str,
    number: u32,
    days_overdue: u32,
}

const INVOICES: [Invoice; 4] = [
    Invoice { customer: "acme", number: 101, days_overdue: 12 },
    Invoice { customer: "globex", number: 102, days_overdue: 0 },
    Invoice { customer: "acme", number: 103, days_overdue: 40 },
    Invoice { customer: "initech", number: 104, days_overdue: 3 },
];

/// Version 1: one email per overdue invoice. Returns the customers it could not reach.
fn send_reminders(invoices: &[Invoice], mailer: &impl Mailer) -> Vec<&'static str> {
    let mut failed = Vec::new();
    for inv in invoices.iter().filter(|i| i.days_overdue > 0) {
        let body = format!("invoice {} is {} days overdue", inv.number, inv.days_overdue);
        if mailer.send(inv.customer, &body).is_err() && !failed.contains(&inv.customer) {
            failed.push(inv.customer);
        }
    }
    failed
}

/// Version 2, after a refactor: one email per customer, listing every overdue invoice.
/// It takes `&dyn Mailer`, so the same doubles work with dynamic dispatch too.
fn send_reminders_batched(invoices: &[Invoice], mailer: &dyn Mailer) -> Vec<&'static str> {
    let mut customers: Vec<&'static str> = Vec::new();
    for inv in invoices.iter().filter(|i| i.days_overdue > 0) {
        if !customers.contains(&inv.customer) {
            customers.push(inv.customer);
        }
    }
    let mut failed = Vec::new();
    for customer in customers {
        let lines: Vec<String> = invoices
            .iter()
            .filter(|i| i.customer == customer && i.days_overdue > 0)
            .map(|i| format!("invoice {} is {} days overdue", i.number, i.days_overdue))
            .collect();
        if mailer.send(customer, &lines.join("; ")).is_err() {
            failed.push(customer);
        }
    }
    failed
}

/// A fake that records every call. `send` takes `&self`, so the record needs a RefCell.
#[derive(Default)]
struct RecordingMailer {
    sent: RefCell<Vec<(String, String)>>,
}

impl Mailer for RecordingMailer {
    fn send(&self, to: &str, body: &str) -> Result<(), MailError> {
        self.sent.borrow_mut().push((to.to_string(), body.to_string()));
        Ok(())
    }
}

/// A fake that fails for one address and succeeds for the rest.
struct Unreachable(&'static str);

impl Mailer for Unreachable {
    fn send(&self, to: &str, _body: &str) -> Result<(), MailError> {
        if to == self.0 { Err(MailError) } else { Ok(()) }
    }
}

/// A fake that plays back a script, one reply per call, and panics if it runs
/// out: what a mocking crate's `times(1)` expectations amount to.
struct ScriptedMailer {
    replies: RefCell<VecDeque<Result<(), MailError>>>,
}

impl Mailer for ScriptedMailer {
    fn send(&self, to: &str, _body: &str) -> Result<(), MailError> {
        self.replies
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| panic!("unexpected call: send({to:?}, ..)"))
    }
}

/// Which version of the reminder code a test runs against.
#[derive(Clone, Copy)]
enum Version {
    PerInvoice,
    PerCustomer,
}

fn run(version: Version, invoices: &[Invoice], mailer: &impl Mailer) -> Vec<&'static str> {
    match version {
        Version::PerInvoice => send_reminders(invoices, mailer),
        Version::PerCustomer => send_reminders_batched(invoices, mailer), // &M coerces to &dyn Mailer
    }
}

/// An outcome test: every overdue invoice was mentioned to the right customer.
fn every_overdue_invoice_was_mentioned(version: Version) {
    let mailer = RecordingMailer::default();
    run(version, &INVOICES, &mailer);
    let sent = mailer.sent.borrow();
    for inv in INVOICES.iter().filter(|i| i.days_overdue > 0) {
        let mentioned = sent
            .iter()
            .any(|(to, body)| to == inv.customer && body.contains(&format!("invoice {}", inv.number)));
        assert!(mentioned, "invoice {} never reached {}", inv.number, inv.customer);
    }
    assert!(sent.iter().all(|(to, _)| to != "globex"), "globex owes nothing");
}

/// An interaction test: send was called exactly three times, in invoice order.
fn send_was_called_three_times(version: Version) {
    let mailer = RecordingMailer::default();
    run(version, &INVOICES, &mailer);
    let to: Vec<String> = mailer.sent.borrow().iter().map(|(to, _)| to.clone()).collect();
    assert_eq!(to, ["acme", "acme", "initech"]);
}

/// A failure path the real mail server would almost never produce on demand.
fn an_unreachable_customer_is_reported(version: Version) {
    assert_eq!(run(version, &INVOICES, &Unreachable("initech")), ["initech"]);
}

/// A scripted test, written by watching version 1 make its calls: the second
/// send fails. The script answers by position, not by customer.
fn scripted_second_send_fails(version: Version) {
    let script = [Ok(()), Err(MailError), Ok(())];
    let mailer = ScriptedMailer { replies: RefCell::new(VecDeque::from(script)) };
    assert_eq!(run(version, &INVOICES, &mailer), ["acme"]);
    let unused = mailer.replies.borrow().len();
    assert_eq!(unused, 0, "{unused} scripted replies were never used");
}

fn check(test: fn(Version), version: Version) -> String {
    let result = panic::catch_unwind(|| test(version));
    match result {
        Ok(()) => "pass".to_string(),
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .map(|s| s.lines().map(str::trim).collect::<Vec<_>>().join("; "))
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            format!("FAIL  {msg}")
        }
    }
}

fn main() {
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    println!("1. The fake records what the code did");
    let mailer = RecordingMailer::default();
    let failed = send_reminders(&INVOICES, &mailer);
    for (to, body) in mailer.sent.borrow().iter() {
        println!("   send({to:?}, {body:?})");
    }
    println!("   failed = {failed:?}");

    println!();
    println!("2. A fake can fail on demand");
    println!("   Unreachable(\"initech\"): failed = {:?}", send_reminders(&INVOICES, &Unreachable("initech")));

    println!();
    println!("3. The same tests against version 1 and the batched version 2");
    let tests: [(&str, fn(Version)); 4] = [
        ("every_overdue_invoice_was_mentioned", every_overdue_invoice_was_mentioned),
        ("an_unreachable_customer_is_reported", an_unreachable_customer_is_reported),
        ("send_was_called_three_times", send_was_called_three_times),
        ("scripted_second_send_fails", scripted_second_send_fails),
    ];
    for (label, version) in [("v1, one email per invoice", Version::PerInvoice), ("v2, one email per customer", Version::PerCustomer)] {
        println!("   {label}");
        for (name, test) in tests {
            println!("     {name:<36} {}", check(test, version));
        }
    }
    println!("   Both versions tell every customer about every overdue invoice.");
    println!("   The outcome tests say so. The two tests that pinned the calls");
    println!("   themselves fail on a refactor that broke nothing a customer sees.");

    panic::set_hook(quiet);
}
