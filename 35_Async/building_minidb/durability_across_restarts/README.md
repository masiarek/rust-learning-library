# Durability across restarts

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A write-ahead log makes a restart harmless by recording every mutating request before it is applied, and group commit makes it affordable by letting many requests share one trip to the disk.

## What it has to cover

- The order that makes it a *write-ahead* log: append the record → sync → apply to the store → acknowledge the client
- What "on disk" means: a successful `write` is not durable until a sync — `sync_data` against `sync_all` — and the cost of one sync, measured on this machine
- **Group commit.** Collect the writes that arrive while one sync is running, sync them together, then acknowledge them all — throughput up, a little latency added to each request; the store actor from chapter 4 is the natural place to batch
- Replay on startup: read the log from the beginning and apply each record in order
- A crash in the middle of an append: detecting a torn final record with a length and a checksum, and truncating it rather than refusing to start
- Why `tokio::fs` runs file IO on the blocking pool, and what that means for a log written on every request
- Where this stops: the log grows forever until a snapshot lets it be truncated — named, not built

## The trap it exists for

Acknowledging the client before the sync returns. It benchmarks beautifully and loses writes the client was told had succeeded, on exactly the crash the log was added for.

## What minidb gains

Data that survives a restart, with a group commit sized by measurement.

## See also

- [Codecs and framing](../codecs_and_framing/README.md) — the frame format the log switches to
- [Who owns the state](../who_owns_the_state/README.md) — the actor that batches the syncs
- [Opening a file](../../../04_Files/opening_a_file/README.md) — the synchronous file API underneath

## Po polsku

**Dziennik zapisu z wyprzedzeniem** (*write-ahead log*, WAL) sprawia, że restart nie gubi danych: każde żądanie zmieniające stan trafia najpierw do dziennika i jest synchronizowane na dysk, a dopiero potem wykonywane i potwierdzane. **Grupowe zatwierdzanie** (*group commit*) zbiera zapisy, które przyszły w trakcie jednej synchronizacji, i utrwala je razem jednym `fsync`, więc zajęty serwer nie płaci jednej podróży na dysk za każde żądanie. Przy starcie dziennik jest odtwarzany od początku.

**Szukaj po polsku:** dziennik zapisu z wyprzedzeniem · grupowe zatwierdzanie · `write ahead log rust` · `group commit fsync`
