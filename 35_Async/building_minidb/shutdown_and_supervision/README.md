# Shutdown and supervision

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** Graceful shutdown is an ordering problem — stop accepting, tell every connection to finish, wait for them, and only then stop the store they write to — and supervision is its other half: deciding what happens when a task ends that nobody asked to stop.

## What it has to cover

- The trigger: Ctrl-C and `SIGTERM` through `tokio::signal`
- [`CancellationToken` ↗](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) to tell every task at once, and [`TaskTracker` ↗](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html) to wait until they have all finished
- **The order.** Stop the accept loop → cancel connections and let each finish its current request → wait for the tracker → close the store's channel so the actor drains what is queued → flush the log (chapter 9) → exit
- A deadline on the drain: how long to wait before aborting whatever is left, and how to report what was cut off
- **Supervision.** What a `JoinHandle`'s result says — finished, panicked, cancelled — and three policies: restart the task, escalate to the whole server, or log and continue
- Why the answer differs by task: a connection task dying is one client's problem; the store actor dying means every later request fails, so it must stop the server

## The trap it exists for

Stopping the store before the connections. Requests still in flight find the channel closed, and a routine deploy turns into writes the clients believe were accepted.

## What minidb gains

A shutdown path that drains in order under a deadline, and a supervisor that restarts nothing silently.

## See also

- [Cancellation](../cancellation/README.md) — the token, first met there
- [Catching a signal](../../../09_Advanced/catching_a_signal/README.md) — what std does and does not give you for `SIGTERM`
- [What a panic costs](../../../17_Option_and_Result/what_a_panic_costs/README.md) — in a thread, only that thread dies; in a task, only that task
- [Supervision ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/communication/supervision/index.html), [structured concurrency ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/async/structured_concurrency/index.html) and [task leak ↗](https://masiarek.github.io/concurrency-learning-library/11_Concepts/hazards/task_leak/index.html) — in the Concurrency library

## Po polsku

**Łagodne zamykanie** (*graceful shutdown*) to przede wszystkim kolejność: przestać przyjmować połączenia, poprosić każde o dokończenie bieżącego żądania, zaczekać na wszystkie i dopiero wtedy zatrzymać magazyn, do którego piszą. **Nadzór** (*supervision*) to druga połowa tematu — decyzja, co zrobić z zadaniem, które zakończyło się samo: uruchomić je ponownie, zatrzymać cały serwer albo tylko zalogować. Śmierć zadania połączenia to problem jednego klienta; śmierć aktora magazynu — wszystkich.

**Szukaj po polsku:** łagodne zamykanie serwera · nadzór nad zadaniami · `tokio graceful shutdown` · `tokio_util TaskTracker`
