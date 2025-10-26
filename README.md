
# 🧩 Taven

> **Taven** — a composable control-flow engine for Rust  
> _“Where logic finds its rhythm.”_

---

### 🌌 Overview

**Taven** is a lightweight, modular engine for building **control-flow systems** —  
behavior trees, state machines, reactive logic pipelines, or anything that can  
be expressed as *nodes* and *flows*.

It provides a clean foundation built around a single idea:

> “Every decision and action can be expressed as a node that drives context.”

---

### ✨ Key Features

- 🧠 **Composable Nodes** — actions, sequences, selectors, parallels  
- 🧩 **Unified Trait Model** — every node implements `Node<Ctx>`  
- ⚙️ **Custom Contexts** — define your own runtime data structure  
- 🧰 **Extensible Design** — build DSLs, editors, or async engines  
- 💾 **Serializable State** — via `serde` and JSON-based `Blackboard`  
- 🧪 **Test-Friendly** — small, deterministic, and side-effect controlled  

---

### 🧱 Workspace Structure

```

taven/
├── Cargo.toml              # workspace root
├── README.md
├── taven-core/             # core logic (Node, Status, Sequence, etc.)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── node.rs
│       ├── context.rs
│       ├── control/
│       │   └── sequence.rs
│       └── leaf/
│           └── action.rs
├── taven-dsl/              # serde-based flow definitions (WIP)
│   ├── Cargo.toml
│   └── src/lib.rs
└── examples/
└── simple.rs

````

---

### 🚀 Quick Start

Add this to your **workspace root**:

```bash
cargo run --example simple
````

or use it as a dependency:

```toml
[dependencies]
taven-core = { git = "https://github.com/yourname/taven", package = "taven-core" }
```

---

### 🧩 Example: Simple AI Flow

```rust
use taven_core::{Action, Sequence, Status};

#[derive(Debug, Default)]
struct AiCtx {
    hp: i32,
    enemy_in_range: bool,
}

fn main() {
    let mut ctx = AiCtx { hp: 30, enemy_in_range: true };

    let check_hp = Action::new(|ctx: &mut AiCtx| {
        if ctx.hp > 50 {
            println!("HP OK ({})", ctx.hp);
            Status::Success
        } else {
            println!("HP low ({})", ctx.hp);
            Status::Failure
        }
    });

    let attack = Action::new(|ctx: &mut AiCtx| {
        if ctx.enemy_in_range {
            println!("Attack!");
            Status::Success
        } else {
            println!("No enemy");
            Status::Failure
        }
    });

    let mut seq = Sequence::new(vec![Box::new(check_hp), Box::new(attack)]);

    match seq.tick(&mut ctx) {
        Status::Success => println!("Sequence succeeded"),
        Status::Failure => println!("Sequence failed"),
        Status::Running => println!("Sequence running"),
    }
}
```

Output:

```
HP low (30)
Sequence failed
```

---

### 🧬 Core Concepts

| Concept        | Description                                                                                |
| -------------- | ------------------------------------------------------------------------------------------ |
| **Node**       | The fundamental executable unit. Implements `fn tick(&mut self, ctx: &mut Ctx) -> Status`. |
| **Status**     | Represents node execution result: `Success`, `Failure`, or `Running`.                      |
| **Context**    | Arbitrary user-defined data passed to all nodes.                                           |
| **Sequence**   | Executes nodes in order until one fails.                                                   |
| **Action**     | Wraps a closure or function that performs logic on the context.                            |
| **Blackboard** | Key-value storage for dynamic runtime data (serde-powered).                                |

---

### 🧭 Roadmap

* [ ] Add **Selector** and **Parallel** control nodes
* [ ] Add **async/await** node execution support
* [ ] Add **builder DSL** (`flow! { ... }` macro)
* [ ] Add **serde-based graph definition** for editor integration
* [ ] Add **taven-editor** (GUI flow builder with Bevy UI)
* [ ] Publish `taven-core` crate to [crates.io](https://crates.io/)

---

### 🧠 Design Philosophy

Taven is built with **engineering clarity** and **compositional elegance** in mind:

> 1. *Everything is a Node.*
> 2. *Nodes know nothing about each other.*
> 3. *Context drives state, not global data.*
> 4. *The flow is data — not code.*

This philosophy makes Taven suitable for AI systems, data pipelines, automation graphs, or any domain requiring controlled, reactive logic execution.

---

### 🛠️ Tech Stack

* **Rust 2021+**
* **Serde / JSON** for state and flow serialization
* **Workspaces** for modular crate design
* **Traits + Dynamic Dispatch** for composability

---

### 🧑‍💻 Contributing

Contributions, ideas, and discussions are welcome!
Open an issue or PR to help shape **Taven** into a robust control-flow framework.

---

### ⚖️ License

MIT License © 2025 — [Your Name or Org]
Freely available for personal and commercial use.

---

### 🌈 Acknowledgements

Inspired by:

* Behavior Trees (AI architecture)
* Finite State Machines
* Rust’s trait-based composability
* Bevy ECS design philosophy

---

> *Taven is still young — but it’s already dreaming of elegant logic.*
> **“Flow. Compose. Repeat.”**

---

