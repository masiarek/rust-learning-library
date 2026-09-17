# Codecs and framing

**Level:** 301 · deep dive

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

**One line:** A length-prefixed frame says how many bytes follow before they arrive, so a value may contain any byte — newlines included — and a decoder that keeps a partial frame in its own buffer makes reading cancel safe by construction.

## What it has to cover

- The restriction the line protocol imposed: a value cannot contain `\n`, and nothing in chapter 3's code says so
- The frame: a fixed-width length in network byte order, then that many bytes of payload
- [`Decoder` ↗](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html): `decode(&mut self, src: &mut BytesMut)` returning `Ok(None)` for "not enough bytes yet", reserving capacity for the rest of the frame, and consuming exactly one frame when it is complete
- **A maximum frame length**, checked before allocating, because the length comes from the client
- `Encoder`, and `Framed` joining both into one value that is a `Stream` of requests and a `Sink` of responses
- **The same codec for the write-ahead log.** A torn record at the end of the file is a partial frame at end of input — chapter 9's checksum and this decoder handle it together
- **Cancel safety, closed.** The partial frame lives in `Framed`'s buffer, not on a dropped future's stack, so a `select!` that drops the read loses nothing — the structural answer to [chapter 5](../cancellation/README.md)

## The trap it exists for

Believing the length prefix. A four-byte length of four gigabytes makes a naive decoder allocate four gigabytes before a single byte of payload has arrived.

## What minidb gains

A binary protocol that carries any value, and one frame format shared by the network and the log.

## See also

- [A length you did not check ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/a_length_you_did_not_check/index.html) — the same frame and the same trap, in the C learning library
- [Byte order on the wire ↗](https://masiarek.github.io/c-learning-library/05_Bytes_on_the_Wire/byte_order_on_the_wire/index.html) — why the length is big-endian
- [Meet the byte](../../../19_Numbers/meet_the_byte/README.md) — what the length field counts
- [Streams, sinks, and pipelining](../streams_sinks_and_pipelining/README.md) — what `Framed` is used for next

## Po polsku

**Ramka z prefiksem długości** mówi, ile bajtów nadejdzie, zanim one nadejdą, więc wartość może zawierać dowolny bajt, także znak nowej linii, którego protokół liniowy nie dopuszczał. **Kodek** (`Decoder` i `Encoder`) trzyma niedokończoną ramkę we własnym buforze — dlatego porzucenie future’a czytającego nic nie gubi. Długości z nagłówka nie wolno ufać: trzeba ją sprawdzić, zanim zarezerwuje się pamięć.

**Szukaj po polsku:** ramkowanie wiadomości · prefiks długości · `tokio_util codec decoder` · `length delimited codec rust`
