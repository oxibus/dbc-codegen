# can-messages-split

This crate demonstrates per-sender code generation from a DBC file using `dbc-codegen`.

## How it works

### Input: `multiple_devices.dbc`

The DBC file defines five nodes (devices) on the CAN bus:

```
BU_: MCU BMS DISPLAY VCU CHARGER
```

Each message has a transmitter (sender):

| Message                        | Sender  |
|--------------------------------|---------|
| `Gear_Selection`               | VCU     |
| `Bike_Information_New`         | VCU     |
| `MCU_Diag_Status`              | MCU     |
| `Cell_voltage_information_BMS` | BMS     |
| `Total_Information_0_BMS`      | BMS     |
| `Charger_to_BMS`               | CHARGER |

### Build step: `build.rs`

At compile time, `build.rs` reads the DBC file and calls `write_split_by_sender`:

```rust
Config::builder()
    .dbc_name("multiple_devices.dbc")
    .dbc_content(&dbc_file)
    .build()
    .write_split_by_sender(&out_dir)
```

This generates one `.rs` file per sender in `OUT_DIR`:

```
OUT_DIR/
├── bms.rs      # Cell_voltage_information_BMS, Total_Information_0_BMS
├── charger.rs  # Charger_to_BMS
├── mcu.rs      # MCU_Diag_Status
├── vcu.rs      # Gear_Selection, Bike_Information_New
└── mod.rs      # glue that includes each file above
```

Each sender file contains:
- A `Messages` enum listing that sender's messages
- A struct per message with typed signal getters and setters
- A `CanError` type for decode errors

The generated `mod.rs` uses inline `include!` macros so Rust can find the files in `OUT_DIR`:

```rust
pub mod vcu { include!(concat!(env!("OUT_DIR"), "/vcu.rs")); }
pub mod mcu { include!(concat!(env!("OUT_DIR"), "/mcu.rs")); }
pub mod bms { include!(concat!(env!("OUT_DIR"), "/bms.rs")); }
pub mod charger { include!(concat!(env!("OUT_DIR"), "/charger.rs")); }
```

### Entry point: `src/lib.rs`

The library simply pulls in the generated `mod.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/mod.rs"));
```

### Using the generated types

After building, each sender's messages are accessible under their module:

```rust
use can_messages_split::vcu::GearSelection;
use can_messages_split::mcu::McuDiagStatus;
use can_messages_split::bms::CellVoltageInformationBms;
use can_messages_split::charger::ChargerToBms;
```

---

## Logic in `dbc-codegen` (`src/lib.rs`)

The split is implemented in three functions in the library.

### 1. `unique_senders`

```rust
fn unique_senders(dbc: &Dbc) -> Vec<String> {
    let mut seen = BTreeSet::new();
    for msg in get_relevant_messages(dbc) {
        if let Transmitter::NodeName(name) = &msg.transmitter {
            seen.insert(name.clone());
        }
    }
    seen.into_iter().collect()
}
```

Walks every message in the DBC and collects the transmitter name when it is a
named node (`NodeName`). Messages with no named transmitter (`VectorXxx`) are
skipped. A `BTreeSet` is used so the result is deduplicated and sorted
alphabetically, giving a stable file order across runs.

### 2. `codegen_for_sender`

```rust
fn codegen_for_sender(&self, out: &mut impl Write, dbc: &Dbc, sender: &str) -> Result<()>
```

Works like the existing `codegen` function but only renders messages belonging
to the given sender:

1. Filters `get_relevant_messages(dbc)` to those whose `transmitter` matches
   `sender`.
2. Writes the standard file header — `DBC_FILE_NAME`, `DBC_FILE_VERSION`, and
   the necessary `use` imports.
3. Calls `render_dbc` with the filtered message slice, which generates the
   `Messages` enum and one struct per message.
4. Appends `CanError` and arbitrary helpers (same as the single-file path).

The raw output is a valid Rust source string, not yet formatted.

### 3. `write_split_by_sender`

```rust
pub fn write_split_by_sender<P: AsRef<Path>>(self, out_dir: P) -> Result<()>
```

Orchestrates the whole split:

1. **Parses** the DBC content once into a `Dbc` value.
2. **Calls `unique_senders`** to get the list of node names (e.g. `["BMS",
   "CHARGER", "MCU", "VCU"]`).
3. **Loops** over each sender:
   - Converts the sender name to `snake_case` using `heck` (e.g. `VCU` →
     `vcu`) to get a valid Rust module name.
   - Calls `codegen_for_sender` into an in-memory buffer.
   - Parses the buffer with `syn` and reformats it with `prettyplease` so the
     output is consistently formatted regardless of how the strings were built.
   - Writes the result to `{out_dir}/{mod_name}.rs`.
4. **Generates `mod.rs`** with one entry per sender using an inline `include!`
   macro instead of a plain `pub mod` declaration. A plain `pub mod vcu;`
   would tell the compiler to look for `vcu.rs` next to `src/lib.rs`, but the
   files are in `OUT_DIR`. The `include!` approach points directly at the right
   path:

   ```rust
   pub mod vcu { include!(concat!(env!("OUT_DIR"), "/vcu.rs")); }
   ```

### Why `render_dbc` takes a message slice

The original `render_dbc` called `get_relevant_messages` internally, so it
always rendered every message. To support the split it was refactored to accept
a `&[&Message]` slice from the caller:

```rust
fn render_dbc(&self, w: &mut impl Write, dbc: &Dbc, messages: &[&Message]) -> Result<()>
```

The single-file path collects all messages and passes the full slice. The
per-sender path passes only the filtered subset. Everything downstream
(`render_root_enum`, `render_message`, etc.) is unaware of the filtering — it
just renders whatever it receives.
