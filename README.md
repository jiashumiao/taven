# Developing

Not available for using now.


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

