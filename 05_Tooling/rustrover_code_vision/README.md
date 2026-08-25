# RustRover Code Vision: the `1 usage` line above your code

**Level:** 101 → 201 · working knowledge

**One line:** The grey `1 usage · 1 implementation` line floating above every declaration is **Code Vision**, a feature separate from the type hints inside your code — right-click it to switch off one metric, uncheck **Enable Code Vision** to switch off all of them, and neither one touches the inlay hints elsewhere in the editor.

```text
1 usage   1 implementation          <- Code Vision, above the declaration
trait Speak {
        1 implementation
    fn say_hello(&self) -> String;
}
```

Every label below is the string shipped in **RustRover 2026.2.1** (build `262.9437.161`), read from the IDE's own resource bundles. Older releases word a few of them differently.

---

## What the numbers say

RustRover's Rust plugin supplies two metrics of its own, and their wording is exactly:

| Count | Usages | Implementations |
|---|---|---|
| 0 | `no usages` | — |
| 1 | `1 usage` | `1 implementation` |
| n | `n usages` | `n implementations` |

Both are links: click the number to jump to the usages or to an implementation. In the settings dialog the second one is listed under the platform's own name, **Inheritors** — Rust has no inheritance, so the plugin prints "implementation" in the editor while the checkbox keeps the platform label.

## Deactivate: right-click the hint itself

The fastest route, and it needs no dialog. Right-click **on the grey text**, not on the code:

- **Hide `Code Vision: Usages` Inlay Hints** — that one metric, everywhere
- **Hide All `Code Vision` Inlay Hints** — the whole feature

The choice is remembered across restarts. There is no undo in that menu, so reactivating is the settings page below.

## Activate and deactivate: the settings page

**RustRover → Settings…** (`⌘,`) → **Editor → Inlay Hints**, then the **Code vision** group.

| Control | Does |
|---|---|
| **Enable Code Vision** | the master switch for every metric |
| **Default position for metrics:** | where hints land — `Top`, `Right`, `Near scroll`, `Empty space` |
| **Visible metrics above declaration:** / **…next to declaration:** | how many fit before the rest collapse into `More…` |
| the **Metric** / **Position** table | one row per metric, checkbox to activate, per-row position override |

Six metrics ship in the table. Four are counts and two only appear after you act:

| Metric | Shows |
|---|---|
| **Usages** | how many places reference this item |
| **Inheritors** | implementations (in Rust), descendants (elsewhere) |
| **Related problems** | project-wide problems tied to this signature — not ones in the same file |
| **Code author** | who last edited it, from VCS. **Off by default** |
| **Rename** | after a rename, offering to update usages |
| **Change signature** | after a signature edit, offering to correct callers |

Unchecking a row deactivates that metric; rechecking it brings the hint straight back — no restart, no reindex.

## Three switches that are not the same switch

This is the part worth getting straight before you go hunting, because turning off the wrong one leaves the `1 usage` line exactly where it was.

| You want gone | Switch | Where |
|---|---|---|
| the `1 usage` line above declarations | **Enable Code Vision** | Settings → Editor → Inlay Hints → Code vision |
| the `: String` type hints inside your code | **Types** under the Rust inlay hints | Settings → Editor → Inlay Hints → Rust |
| every inlay in the IDE, both of the above | **Toggle Inlay Hints Globally** | Find Action (`⌘⇧A`) |
| Code Vision, but only in this project | **Toggle Code Vision for Project** | Find Action (`⌘⇧A`) |

The two Find Action entries have no default shortcut. If you flip either one often, assign one in **Settings → Keymap** — the global toggle is the one worth a key, because it is the only switch that clears the editor completely for a screenshot or a screen share.

## Where the answer is stored

Both feature settings persist per IDE version, not per project:

```sh
~/Library/Application\ Support/JetBrains/RustRover2026.2/options/editor.xml
```

Inlay hint providers you have switched off appear as a set of ids — this is a machine with the Rust type hints turned off and Code Vision untouched:

```xml
<component name="InlayHintsSettings">
  <option name="disabledHintProviderIds">
    <set>
      <option value="Rust.rust.type.hints" />
    </set>
  </option>
</component>
```

The file records **only what differs from the defaults**, so an absent `CodeVisionSettings` component means every metric is at its shipped state rather than that the feature is off. RustRover writes it on exit, so read it with the IDE closed if you want it current.

## Should you turn it off?

Two honest arguments, and the answer is per person rather than per project.

**Keep it.** The usage count is the cheapest dead-code detector in the window — `no usages` on a `pub fn` you thought was wired up is a finding, and it arrives without running anything. On a trait it is the fastest route to the implementations, which in Rust are scattered by design rather than gathered in a class body.

**Drop it.** It inserts a line above nearly every declaration, so a screenful of short functions can be a third hint text, and the counts move as you type. If you are reading rather than navigating, or recording a screen, it is noise. `Position: Right` is the middle setting most people land on — the counts move onto the declaration's own line and the vertical rhythm comes back.

## See also

- [RustRover setup](../rustrover_setup/README.md) — clippy as the on-the-fly linter, and run configurations in a workspace
- [Choosing an editor](../editors/README.md) — why every editor but this one is a front end for `rust-analyzer`
- [TOOLCHAIN.md](../../TOOLCHAIN.md) — the map of these pages
