# Why observability matters

**Level:** 101 → 201 · for newcomers

**One line:** Monitoring tells you a threshold was crossed; observability tells you which request crossed it and where it spent its nine seconds.

> **Stub — an outline, not a lesson.** There is no runnable example behind this page yet, so nothing on it has been through [the check that backs every other claim in this library](../../CONTRIBUTING.md). The bullets below are the questions the finished page has to answer.

## What it has to cover

- The two questions, and why only one of them can be prepared in advance. A dashboard answers *is the error rate up* — somebody chose that number months ago. An investigation answers *why did this checkout take nine seconds when the same code took 200 ms an hour ago*, and nobody chose that question in advance because nobody knew it would be asked.
- What a distributed failure looks like from inside one service: your handler returned `Ok` in 4 ms, the user waited nine seconds, and every line in your log is true and none of them is the answer.
- Averages hide exactly the case you are paged about — p50 fine, p99 terrible, one customer generating all of it. The mean is the statistic most likely to say nothing is wrong.
- The word *observable* is borrowed from control theory, where it means the internal state can be reconstructed from the outputs. The useful half of the analogy: the outputs have to be designed in. You do not discover them during the incident.
- What it costs, said up front rather than in a footnote: every signal is data somebody stores, ships and pays for. That is what [sampling](../sampling/README.md) and [cardinality](../metrics_and_cardinality/README.md) are about, and they are not optional extras.

## The trap it exists for

*"We already have logs."* You have a service's own small truth, with nothing joining it to the truth of the service that called it. Correlation is the whole feature, and it is the one thing a `println!` cannot add afterwards — by the time you want the id, the request is over and the line is written.

## See also

- [The three pillars](../the_three_pillars/README.md) — the three signals this page is arguing for
- [What to instrument first](../what_to_instrument/README.md) — the same argument, reduced to a checklist
- [Clients](../../07_Clients/README.md) — the section where the network first shows up as something that can say no

## Po polsku

Słowo **obserwowalność** (*observability*) nie jest w polszczyźnie kalką z żargonu DevOps — siedzi w naszej literaturze technicznej od dziesięcioleci, tyle że w teorii sterowania: układ jest obserwowalny, jeśli jego stan wewnętrzny da się odtworzyć z przebiegu wyjść. Kto miał automatykę na studiach, ma tu gotową i całkiem trafną intuicję, z jednym zastrzeżeniem, które ta strona stawia wprost: wyjścia trzeba **zaprojektować wcześniej**. W trakcie awarii już się ich nie dorobi. I to jest cała różnica między dwoma pytaniami: „czy wzrósł odsetek błędów” ktoś zapisał jako próg pół roku temu, a „dlaczego **to jedno** zamówienie szło dziewięć sekund, skoro godzinę wcześniej ten sam kod robił to w 200 ms” zadajesz o trzeciej w nocy i nikt go wcześniej nie przewidział.

Dwie rzeczy brzmią po polsku ostrzej niż po angielsku i warto z tego skorzystać. Po pierwsze statystyka: **mediana** (p50) wygląda przyzwoicie dokładnie wtedy, gdy p99 jest tragiczny, a **średnia** jest z tych trzech liczb najbardziej skłonna zapewnić, że nic się nie dzieje — tymczasem twój handler zwrócił `Ok` w 4 ms, użytkownik czekał dziewięć sekund, i każda linijka w logu jest prawdziwa, tylko żadna nie jest odpowiedzią. Po drugie pułapka „przecież mamy logi”: nie brakuje wtedy **zapisów**, brakuje **korelacji**. Log jednej usługi to jej własna mała prawda, której nic nie łączy z prawdą usługi wywołującej — a identyfikatora żądania `println!` nie dopisze po fakcie, bo kiedy zaczynasz go potrzebować, żądanie już się skończyło i linia jest zapisana.

**Szukaj po polsku:** obserwowalność układu · korelacja logów · identyfikator żądania · `observability vs monitoring` · `p99 latency tail`
