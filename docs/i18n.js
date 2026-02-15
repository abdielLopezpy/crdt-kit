/* ============================================================
   crdt-kit — i18n + Interactive Demos
   ============================================================ */

// ==================== TRANSLATIONS ====================
const T = {
  en: {
    // Nav
    "nav.features": "Features",
    "nav.types": "Types",
    "nav.quickstart": "Quick Start",
    "nav.architecture": "Architecture",
    "nav.ecosystem": "Ecosystem",

    // Hero
    "hero.tagline": "Conflict-Free Replicated Data Types for Rust",
    "hero.sub": 'Lightweight (~50KB), <code>no_std</code> compatible, optimized for IoT, edge computing, WASM, and local-first architectures.',
    "hero.getstarted": "Get Started",
    "hero.docs": "API Documentation",

    // What
    "what.title": "What are CRDTs?",
    "what.p1": '<strong>Conflict-Free Replicated Data Types</strong> are data structures that can be replicated across multiple devices, updated independently and concurrently, and merged automatically without conflicts.',
    "what.p2": 'Unlike traditional databases that require a central server to coordinate writes, CRDTs guarantee that all replicas will <strong>converge to the same state</strong> regardless of the order in which updates are received. This guarantee is mathematical, not dependent on network conditions.',
    "what.p3": 'This makes them ideal for <strong>offline-first apps</strong>, <strong>peer-to-peer systems</strong>, <strong>IoT networks</strong>, and any scenario where devices need to work independently and sync later.',

    // Why
    "why.title": "Why crdt-kit?",
    "why.desc": "Built specifically for resource-constrained, latency-sensitive environments where existing solutions add too much overhead.",
    "why.stat1": "Binary size (core)",
    "why.stat2": "Dependencies (core)",
    "why.stat3": "CRDT types",
    "why.stat4": "Tests",
    "why.stat5": "ops/sec peak",

    // Features
    "features.title": "Features",
    "features.nostd": 'Runs on bare metal, Raspberry Pi, ESP32. All types work with <code>#![no_std]</code> + <code>alloc</code>. No heap allocator required for core operations.',
    "features.delta_title": "Delta Sync",
    "features.delta": 'Only send what changed since the last sync. <code>DeltaCrdt</code> trait for GCounter, PNCounter, and ORSet. Minimizes bandwidth on LoRa, BLE, and constrained networks.',
    "features.wasm": 'First-class WebAssembly bindings via <code>wasm-bindgen</code>. Use the same CRDTs in browser, Deno, Node.js, and Rust backend. GCounter, PNCounter, LWWRegister, GSet, ORSet, TextCrdt.',
    "features.migrate_title": "Schema Migrations",
    "features.migrate": 'Transparent, lazy migrations on read. <code>#[crdt_schema]</code> + <code>#[migration]</code> proc macros. Linear chain v1 &rarr; v2 &rarr; v3. Deterministic &mdash; two devices migrating the same data produce identical results.',
    "features.storage_title": "Persistent Storage",
    "features.storage": "Three backends, one trait surface: SQLite (bundled), redb (pure Rust, no C deps), and in-memory. Event sourcing, snapshots, and compaction built in.",
    "features.codegen_title": "Code Generation",
    "features.codegen": 'Define entities in TOML, run <code>crdt generate</code>, get a complete persistence layer: versioned structs, migration functions, repository traits, event sourcing, delta sync.',
    "features.serde": '<code>Serialize</code> / <code>Deserialize</code> for all 9 CRDT types via feature flag. Works with JSON, MessagePack, Postcard, CBOR &mdash; any serde-compatible format.',
    "features.events_title": "Event Sourcing",
    "features.events": 'Full event log with append, replay, snapshots, and compaction. <code>EventStore</code> trait implemented by all backends. Configurable snapshot policies.',
    "features.devtools_title": "Developer Tools",
    "features.devtools": 'CLI with <code>status</code>, <code>inspect</code>, <code>compact</code>, <code>export</code>, <code>generate</code>, <code>dev-ui</code>. Embedded dark-themed web panel for visual database inspection.',

    // Comparison
    "compare.title": "How It Compares",
    "compare.lang": "Language",
    "compare.deps": "Zero dependencies (core)",
    "compare.nostd": "no_std / embedded",
    "compare.storage": "Persistent storage",
    "compare.migrations": "Schema migrations",
    "compare.codegen": "Code generation",
    "compare.size": "Binary size",

    // CRDTs
    "crdts.title": "9 CRDT Types",
    "crdts.desc": 'From simple counters to collaborative text editing. Every type is <code>Send + Sync</code>, <code>serde</code>-ready, and mathematically convergent.',
    "crdts.counters": "Counters",
    "crdts.registers": "Registers",
    "crdts.sets": "Sets",
    "crdts.sequences": "Sequences",
    "crdts.gcounter": "Grow-only counter. Each node maintains its own count; the total is the sum of all nodes. Monotonically increasing.",
    "crdts.gcounter_use": "Page views, IoT sensor events, download counts, heartbeats",
    "crdts.pncounter": "Positive-negative counter. Supports both increment and decrement operations. Internal pair of GCounters.",
    "crdts.pncounter_use": "Inventory stock, likes/dislikes, seat reservations, voting",
    "crdts.lww": "Last-Writer-Wins register. On concurrent writes, the one with the highest timestamp wins. Uses Hybrid Logical Clock.",
    "crdts.lww_use": "User profile fields, GPS location, config settings, status",
    "crdts.mv": "Multi-Value register. Preserves all concurrent writes instead of silently discarding. Application resolves conflicts.",
    "crdts.mv_use": "Version tracking, audit trails, conflict visualization",
    "crdts.gset": "Grow-only set. Elements can be added but never removed. Union on merge.",
    "crdts.gset_use": "Seen message IDs, tags, audit logs",
    "crdts.twopset": "Two-Phase set. Add and remove, but once removed an element cannot be re-added.",
    "crdts.twopset_use": "Blocklists, revoked tokens, bans",
    "crdts.orset": "Observed-Remove set. Add and remove elements freely. Concurrent add+remove: add wins.",
    "crdts.orset_use": "Shopping carts, chat members, todo lists",
    "crdts.rga": "Replicated Growable Array. Ordered sequence with insert, remove, and move. Position-independent identifiers.",
    "crdts.rga_use": "Playlists, kanban boards, ordered lists, timelines",
    "crdts.text": 'Collaborative text editing. <code>insert_str()</code>, <code>remove_range()</code>, <code>fork()</code>. Built on RGA with string optimizations.',
    "crdts.text_use": "Google Docs-style editing, shared notes, wikis",
    "crdts.crdt_trait": 'Core merge semantics. <code>merge(&amp;other)</code> is commutative, associative, and idempotent. Implemented by all 9 types.',
    "crdts.delta_trait": 'Efficient delta sync. <code>delta(&amp;other)</code> computes the minimal diff; <code>apply_delta()</code> applies it. GCounter, PNCounter, ORSet.',

    // Quick Start
    "qs.title": "Quick Start",
    "qs.basic": "Basic Usage",
    "qs.persistence": "With Persistence",
    "qs.migrations": "Schema Migrations",
    "qs.codegen": "Code Generation",

    // Architecture
    "arch.title": "Architecture",
    "arch.desc": "Multi-crate workspace. Each crate is independently versioned and published. Use only what you need.",
    "arch.step1_title": "Define",
    "arch.step1": 'Write a <code>crdt-schema.toml</code> with your entities, versions, CRDT fields, and relations.',
    "arch.step2_title": "Generate",
    "arch.step2": 'Run <code>crdt generate</code>. Get models, migrations, repository traits, event types, and sync functions.',
    "arch.step3_title": "Use",
    "arch.step3": 'Import the generated <code>Persistence&lt;S&gt;</code> in your app. Access repositories, store data, sync between nodes.',

    // Use Cases
    "use.title": "Use Cases",
    "use.iot_title": "IoT & Sensors",
    "use.iot": '<code>no_std</code> core runs on ESP32, Raspberry Pi, bare metal. Delta sync minimizes bandwidth over LoRa/BLE. SQLite or redb for local persistence. Schema migrations handle OTA firmware updates seamlessly.',
    "use.mobile_title": "Mobile Apps",
    "use.mobile": "Offline-first architecture. Users edit data without network. When connectivity returns, all changes merge automatically. No conflict dialogs, no data loss.",
    "use.collab_title": "Real-time Collaboration",
    "use.collab": "TextCrdt for Google Docs-style editing. ORSet for shared collections. MVRegister to show concurrent edits. No central coordinator required.",
    "use.edge_title": "Edge Computing",
    "use.edge": "Deploy CRDTs at CDN edge nodes. Each node processes writes locally. Delta sync propagates changes between edges with minimal bandwidth. Pure-Rust redb backend \u2014 no C dependencies to cross-compile.",
    "use.p2p_title": "P2P Networks",
    "use.p2p": "No central server. Every peer is equal. Sync via any transport: WebSocket, WebRTC, Bluetooth, USB drive. Merge is commutative, associative, idempotent \u2014 order and duplicates don't matter.",
    "use.wasm_title": "WASM & Browser",
    "use.wasm": 'Same CRDT logic runs in Rust backend and browser via WebAssembly. <code>wasm-bindgen</code> bindings for GCounter, PNCounter, LWWRegister, GSet, ORSet, TextCrdt. Ship one codebase everywhere.',

    // Ecosystem
    "eco.title": "Ecosystem",
    "eco.desc": "7 crates, independently versioned and published on crates.io. Use only what you need.",
    "eco.kit": 'Core library. 9 CRDT types, Hybrid Logical Clock, <code>Crdt</code> and <code>DeltaCrdt</code> traits. Zero dependencies in default config.',
    "eco.store": 'Persistence. SQLite (bundled), redb (pure Rust), memory. <code>StateStore</code>, <code>EventStore</code>, <code>BatchOps</code>. High-level <code>CrdtDb</code> API.',
    "eco.migrate": 'Versioned serialization. Version envelopes, transparent lazy migration on read, <code>#[crdt_schema]</code> + <code>#[migration]</code> proc macros.',
    "eco.codegen": "Code generation from TOML schemas. Generates models, migrations, repositories, events, sync, and store factory.",
    "eco.cli": '<code>crdt</code> binary: <code>generate</code>, <code>dev-ui</code>, <code>status</code>, <code>inspect</code>, <code>compact</code>, <code>export</code>. All developer workflows in one tool.',
    "eco.devui": "Embedded web panel. Dark-themed Axum app for browsing namespaces, entities, event logs, version envelopes, and snapshots.",

    // Performance
    "perf.title": "Performance",
    "perf.desc": 'Measured with Criterion on optimized builds. All types are <code>Send + Sync</code> for safe concurrent access.',
    "perf.op": "Operation",
    "perf.time": "Time",
    "perf.throughput": "Throughput",

    // Guarantees
    "guar.title": "Mathematical Guarantees",
    "guar.desc": 'All CRDTs satisfy <strong>Strong Eventual Consistency (SEC)</strong>. Verified by 268 tests across the workspace.',
    "guar.comm": "Commutativity",
    "guar.comm_desc": "Order of sync doesn't matter. Device A syncing with B, or B with A, yields the same result.",
    "guar.assoc": "Associativity",
    "guar.assoc_desc": "Group syncs however you want. Merge A+B first then C, or B+C first then A \u2014 same result.",
    "guar.idem": "Idempotency",
    "guar.idem_desc": "Safe to retry. Duplicate messages, network replays, re-syncs \u2014 no double-counting, no corruption.",

    // Examples
    "ex.title": "Examples",
    "ex.desc": "Ready-to-run examples covering every feature.",
    "ex.basics": "CRDT Basics",
    "ex.persist": "Persistence & Migration",
    "ex.fullstack": "Full-Stack Example",
    "ex.fullstack_note": "Complete app: codegen + migrations + CRDT fields + entity relations + delta sync + event sourcing",

    // CLI
    "cli.title": "Developer CLI",
    "cli.status": "Database overview: namespaces, key counts, storage size, event log stats.",
    "cli.inspect": "Entity detail: current value, version envelope, full event log.",
    "cli.compact": "Snapshot + truncate event logs. Reclaim storage space.",
    "cli.export": "JSON export of all entities in a namespace. Pipe to jq, scripts, backups.",
    "cli.generate": 'Generate persistence layer from TOML schema. <code>--dry-run</code> to preview.',
    "cli.devui": 'Launch dark-themed web panel at <code>localhost:4242</code>. Browse everything visually.',

    // Interactive demo
    "demo.title": "Interactive Demo",
    "demo.desc": "See CRDTs in action. Click the buttons to simulate operations on distributed nodes.",
    "demo.tab_counter": "GCounter",
    "demo.tab_set": "ORSet",
    "demo.tab_register": "LWWRegister",

    // CTA
    "cta.title": "Start building offline-first",
    "cta.docs": "API Documentation",

    // Footer
    "footer.devguide": "Development Guide",
  },

  es: {
    // Nav
    "nav.features": "Funcionalidades",
    "nav.types": "Tipos",
    "nav.quickstart": "Inicio",
    "nav.architecture": "Arquitectura",
    "nav.ecosystem": "Ecosistema",

    // Hero
    "hero.tagline": "Tipos de Datos Replicados Libres de Conflictos para Rust",
    "hero.sub": 'Ligero (~50KB), compatible con <code>no_std</code>, optimizado para IoT, edge computing, WASM y arquitecturas local-first.',
    "hero.getstarted": "Comenzar",
    "hero.docs": "Documentaci\u00f3n API",

    // What
    "what.title": "\u00bfQu\u00e9 son los CRDTs?",
    "what.p1": 'Los <strong>Conflict-Free Replicated Data Types</strong> (Tipos de Datos Replicados Libres de Conflictos) son estructuras de datos que pueden replicarse en m\u00faltiples dispositivos, actualizarse de forma independiente y concurrente, y fusionarse autom\u00e1ticamente sin conflictos.',
    "what.p2": 'A diferencia de las bases de datos tradicionales que requieren un servidor central para coordinar escrituras, los CRDTs garantizan que todas las r\u00e9plicas <strong>converger\u00e1n al mismo estado</strong> sin importar el orden en que se reciban las actualizaciones. Esta garant\u00eda es matem\u00e1tica, no depende de condiciones de red.',
    "what.p3": 'Esto los hace ideales para <strong>apps offline-first</strong>, <strong>sistemas peer-to-peer</strong>, <strong>redes IoT</strong>, y cualquier escenario donde los dispositivos necesiten trabajar independientemente y sincronizar despu\u00e9s.',

    // Why
    "why.title": "\u00bfPor qu\u00e9 crdt-kit?",
    "why.desc": "Construido espec\u00edficamente para entornos con recursos limitados y sensibles a la latencia donde las soluciones existentes agregan demasiado overhead.",
    "why.stat1": "Tama\u00f1o binario (core)",
    "why.stat2": "Dependencias (core)",
    "why.stat3": "Tipos CRDT",
    "why.stat4": "Tests",
    "why.stat5": "ops/seg pico",

    // Features
    "features.title": "Funcionalidades",
    "features.nostd": 'Corre en bare metal, Raspberry Pi, ESP32. Todos los tipos funcionan con <code>#![no_std]</code> + <code>alloc</code>. Sin necesidad de heap allocator para operaciones b\u00e1sicas.',
    "features.delta_title": "Sync por Deltas",
    "features.delta": 'Solo env\u00eda lo que cambi\u00f3 desde la \u00faltima sincronizaci\u00f3n. Trait <code>DeltaCrdt</code> para GCounter, PNCounter y ORSet. Minimiza ancho de banda en LoRa, BLE y redes limitadas.',
    "features.wasm": 'Bindings WebAssembly de primera clase v\u00eda <code>wasm-bindgen</code>. Usa los mismos CRDTs en navegador, Deno, Node.js y backend Rust. GCounter, PNCounter, LWWRegister, GSet, ORSet, TextCrdt.',
    "features.migrate_title": "Migraciones de Schema",
    "features.migrate": 'Migraciones transparentes y lazy al leer. Proc macros <code>#[crdt_schema]</code> + <code>#[migration]</code>. Cadena lineal v1 &rarr; v2 &rarr; v3. Determin\u00edstico \u2014 dos dispositivos migrando los mismos datos producen resultados id\u00e9nticos.',
    "features.storage_title": "Almacenamiento Persistente",
    "features.storage": "Tres backends, una interfaz de traits: SQLite (bundled), redb (Rust puro, sin deps C), y en memoria. Event sourcing, snapshots y compactaci\u00f3n incluidos.",
    "features.codegen_title": "Generaci\u00f3n de C\u00f3digo",
    "features.codegen": 'Define entidades en TOML, ejecuta <code>crdt generate</code>, obt\u00e9n una capa de persistencia completa: structs versionados, funciones de migraci\u00f3n, traits de repositorio, event sourcing, delta sync.',
    "features.serde": '<code>Serialize</code> / <code>Deserialize</code> para los 9 tipos CRDT v\u00eda feature flag. Funciona con JSON, MessagePack, Postcard, CBOR \u2014 cualquier formato compatible con serde.',
    "features.events_title": "Event Sourcing",
    "features.events": 'Log de eventos completo con append, replay, snapshots y compactaci\u00f3n. Trait <code>EventStore</code> implementado por todos los backends. Pol\u00edticas de snapshot configurables.',
    "features.devtools_title": "Herramientas Dev",
    "features.devtools": 'CLI con <code>status</code>, <code>inspect</code>, <code>compact</code>, <code>export</code>, <code>generate</code>, <code>dev-ui</code>. Panel web embebido con tema oscuro para inspecci\u00f3n visual de base de datos.',

    // Comparison
    "compare.title": "Comparaci\u00f3n",
    "compare.lang": "Lenguaje",
    "compare.deps": "Cero dependencias (core)",
    "compare.nostd": "no_std / embebido",
    "compare.storage": "Almacenamiento persistente",
    "compare.migrations": "Migraciones de schema",
    "compare.codegen": "Generaci\u00f3n de c\u00f3digo",
    "compare.size": "Tama\u00f1o binario",

    // CRDTs
    "crdts.title": "9 Tipos de CRDT",
    "crdts.desc": 'Desde contadores simples hasta edici\u00f3n colaborativa de texto. Todos los tipos son <code>Send + Sync</code>, compatibles con <code>serde</code>, y matem\u00e1ticamente convergentes.',
    "crdts.counters": "Contadores",
    "crdts.registers": "Registros",
    "crdts.sets": "Conjuntos",
    "crdts.sequences": "Secuencias",
    "crdts.gcounter": "Contador solo-crecimiento. Cada nodo mantiene su propia cuenta; el total es la suma de todos los nodos. Mon\u00f3tonamente creciente.",
    "crdts.gcounter_use": "Visitas de p\u00e1gina, eventos IoT, conteo de descargas, heartbeats",
    "crdts.pncounter": "Contador positivo-negativo. Soporta operaciones de incremento y decremento. Par interno de GCounters.",
    "crdts.pncounter_use": "Stock de inventario, likes/dislikes, reservas de asientos, votaci\u00f3n",
    "crdts.lww": "Registro Last-Writer-Wins. En escrituras concurrentes, gana la que tiene el timestamp m\u00e1s alto. Usa Hybrid Logical Clock.",
    "crdts.lww_use": "Campos de perfil de usuario, ubicaci\u00f3n GPS, configuraci\u00f3n, estado",
    "crdts.mv": "Registro Multi-Valor. Preserva todas las escrituras concurrentes en vez de descartarlas silenciosamente. La aplicaci\u00f3n resuelve conflictos.",
    "crdts.mv_use": "Rastreo de versiones, auditor\u00eda, visualizaci\u00f3n de conflictos",
    "crdts.gset": "Conjunto solo-crecimiento. Los elementos se pueden agregar pero nunca eliminar. Uni\u00f3n al hacer merge.",
    "crdts.gset_use": "IDs de mensajes vistos, tags, logs de auditor\u00eda",
    "crdts.twopset": "Conjunto de dos fases. Agregar y eliminar, pero una vez eliminado un elemento no puede re-agregarse.",
    "crdts.twopset_use": "Listas de bloqueo, tokens revocados, bans",
    "crdts.orset": "Conjunto Observed-Remove. Agregar y eliminar elementos libremente. Add+remove concurrente: gana add.",
    "crdts.orset_use": "Carritos de compras, miembros de chat, listas de tareas",
    "crdts.rga": "Replicated Growable Array. Secuencia ordenada con insert, remove y move. Identificadores independientes de posici\u00f3n.",
    "crdts.rga_use": "Playlists, tableros kanban, listas ordenadas, timelines",
    "crdts.text": 'Edici\u00f3n colaborativa de texto. <code>insert_str()</code>, <code>remove_range()</code>, <code>fork()</code>. Construido sobre RGA con optimizaciones para strings.',
    "crdts.text_use": "Edici\u00f3n estilo Google Docs, notas compartidas, wikis",
    "crdts.crdt_trait": 'Sem\u00e1ntica de merge core. <code>merge(&amp;other)</code> es conmutativo, asociativo e idempotente. Implementado por los 9 tipos.',
    "crdts.delta_trait": 'Sync eficiente por deltas. <code>delta(&amp;other)</code> computa el diff m\u00ednimo; <code>apply_delta()</code> lo aplica. GCounter, PNCounter, ORSet.',

    // Quick Start
    "qs.title": "Inicio R\u00e1pido",
    "qs.basic": "Uso B\u00e1sico",
    "qs.persistence": "Con Persistencia",
    "qs.migrations": "Migraciones de Schema",
    "qs.codegen": "Generaci\u00f3n de C\u00f3digo",

    // Architecture
    "arch.title": "Arquitectura",
    "arch.desc": "Workspace multi-crate. Cada crate tiene versionado y publicaci\u00f3n independiente. Usa solo lo que necesites.",
    "arch.step1_title": "Definir",
    "arch.step1": 'Escribe un <code>crdt-schema.toml</code> con tus entidades, versiones, campos CRDT y relaciones.',
    "arch.step2_title": "Generar",
    "arch.step2": 'Ejecuta <code>crdt generate</code>. Obt\u00e9n modelos, migraciones, traits de repositorio, tipos de eventos y funciones de sync.',
    "arch.step3_title": "Usar",
    "arch.step3": 'Importa el <code>Persistence&lt;S&gt;</code> generado en tu app. Accede a repositorios, almacena datos, sincroniza entre nodos.',

    // Use Cases
    "use.title": "Casos de Uso",
    "use.iot_title": "IoT y Sensores",
    "use.iot": 'Core <code>no_std</code> corre en ESP32, Raspberry Pi, bare metal. Delta sync minimiza ancho de banda en LoRa/BLE. SQLite o redb para persistencia local. Las migraciones de schema manejan actualizaciones OTA de firmware sin problemas.',
    "use.mobile_title": "Apps M\u00f3viles",
    "use.mobile": "Arquitectura offline-first. Los usuarios editan datos sin red. Cuando la conectividad regresa, todos los cambios se fusionan autom\u00e1ticamente. Sin di\u00e1logos de conflicto, sin p\u00e9rdida de datos.",
    "use.collab_title": "Colaboraci\u00f3n en Tiempo Real",
    "use.collab": "TextCrdt para edici\u00f3n estilo Google Docs. ORSet para colecciones compartidas. MVRegister para mostrar ediciones concurrentes. Sin coordinador central requerido.",
    "use.edge_title": "Edge Computing",
    "use.edge": "Despliega CRDTs en nodos edge de CDN. Cada nodo procesa escrituras localmente. Delta sync propaga cambios entre edges con m\u00ednimo ancho de banda. Backend redb de Rust puro \u2014 sin dependencias C para cross-compilar.",
    "use.p2p_title": "Redes P2P",
    "use.p2p": "Sin servidor central. Cada peer es igual. Sync v\u00eda cualquier transporte: WebSocket, WebRTC, Bluetooth, USB. Merge es conmutativo, asociativo, idempotente \u2014 el orden y los duplicados no importan.",
    "use.wasm_title": "WASM y Navegador",
    "use.wasm": 'La misma l\u00f3gica CRDT corre en backend Rust y en el navegador v\u00eda WebAssembly. Bindings <code>wasm-bindgen</code> para GCounter, PNCounter, LWWRegister, GSet, ORSet, TextCrdt. Env\u00eda un solo codebase a todos lados.',

    // Ecosystem
    "eco.title": "Ecosistema",
    "eco.desc": "7 crates, versionados y publicados independientemente en crates.io. Usa solo lo que necesites.",
    "eco.kit": 'Librer\u00eda core. 9 tipos CRDT, Hybrid Logical Clock, traits <code>Crdt</code> y <code>DeltaCrdt</code>. Cero dependencias en configuraci\u00f3n por defecto.',
    "eco.store": 'Persistencia. SQLite (bundled), redb (Rust puro), memoria. <code>StateStore</code>, <code>EventStore</code>, <code>BatchOps</code>. API de alto nivel <code>CrdtDb</code>.',
    "eco.migrate": 'Serializaci\u00f3n versionada. Envoltorios de versi\u00f3n, migraci\u00f3n lazy transparente al leer, proc macros <code>#[crdt_schema]</code> + <code>#[migration]</code>.',
    "eco.codegen": "Generaci\u00f3n de c\u00f3digo desde schemas TOML. Genera modelos, migraciones, repositorios, eventos, sync y factory de store.",
    "eco.cli": 'Binario <code>crdt</code>: <code>generate</code>, <code>dev-ui</code>, <code>status</code>, <code>inspect</code>, <code>compact</code>, <code>export</code>. Todos los flujos de desarrollo en una herramienta.',
    "eco.devui": "Panel web embebido. App Axum con tema oscuro para navegar namespaces, entidades, logs de eventos, envoltorios de versi\u00f3n y snapshots.",

    // Performance
    "perf.title": "Rendimiento",
    "perf.desc": 'Medido con Criterion en builds optimizados. Todos los tipos son <code>Send + Sync</code> para acceso concurrente seguro.',
    "perf.op": "Operaci\u00f3n",
    "perf.time": "Tiempo",
    "perf.throughput": "Throughput",

    // Guarantees
    "guar.title": "Garant\u00edas Matem\u00e1ticas",
    "guar.desc": 'Todos los CRDTs satisfacen <strong>Strong Eventual Consistency (SEC)</strong>. Verificado por 268 tests en todo el workspace.',
    "guar.comm": "Conmutatividad",
    "guar.comm_desc": "El orden de sincronizaci\u00f3n no importa. Dispositivo A sincronizando con B, o B con A, produce el mismo resultado.",
    "guar.assoc": "Asociatividad",
    "guar.assoc_desc": "Agrupa los syncs como quieras. Merge A+B primero luego C, o B+C primero luego A \u2014 mismo resultado.",
    "guar.idem": "Idempotencia",
    "guar.idem_desc": "Seguro de reintentar. Mensajes duplicados, replays de red, re-syncs \u2014 sin doble conteo, sin corrupci\u00f3n.",

    // Examples
    "ex.title": "Ejemplos",
    "ex.desc": "Ejemplos listos para ejecutar cubriendo cada funcionalidad.",
    "ex.basics": "B\u00e1sicos de CRDT",
    "ex.persist": "Persistencia y Migraci\u00f3n",
    "ex.fullstack": "Ejemplo Completo",
    "ex.fullstack_note": "App completa: codegen + migraciones + campos CRDT + relaciones entre entidades + delta sync + event sourcing",

    // CLI
    "cli.title": "CLI para Desarrolladores",
    "cli.status": "Vista general de la base de datos: namespaces, conteo de keys, tama\u00f1o de almacenamiento, estad\u00edsticas del event log.",
    "cli.inspect": "Detalle de entidad: valor actual, envoltorio de versi\u00f3n, event log completo.",
    "cli.compact": "Snapshot + truncar event logs. Recuperar espacio de almacenamiento.",
    "cli.export": "Exportar JSON de todas las entidades en un namespace. Pipe a jq, scripts, backups.",
    "cli.generate": 'Generar capa de persistencia desde schema TOML. <code>--dry-run</code> para previsualizar.',
    "cli.devui": 'Lanzar panel web con tema oscuro en <code>localhost:4242</code>. Navegar todo visualmente.',

    // Interactive demo
    "demo.title": "Demo Interactivo",
    "demo.desc": "Mira los CRDTs en acci\u00f3n. Haz clic en los botones para simular operaciones en nodos distribuidos.",
    "demo.tab_counter": "GCounter",
    "demo.tab_set": "ORSet",
    "demo.tab_register": "LWWRegister",

    // CTA
    "cta.title": "Empieza a construir offline-first",
    "cta.docs": "Documentaci\u00f3n API",

    // Footer
    "footer.devguide": "Gu\u00eda de Desarrollo",
  }
};

// ==================== i18n ENGINE ====================
let currentLang = localStorage.getItem('crdt-kit-lang') || 'en';

function setLang(lang) {
  currentLang = lang;
  localStorage.setItem('crdt-kit-lang', lang);
  document.documentElement.setAttribute('data-lang', lang);
  document.documentElement.lang = lang;

  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    const text = T[lang]?.[key];
    if (text) el.innerHTML = text;
  });
}

// Init language
setLang(currentLang);

// Toggle button
document.getElementById('langToggle')?.addEventListener('click', () => {
  setLang(currentLang === 'en' ? 'es' : 'en');
});

// ==================== COPY BUTTONS ====================
document.querySelectorAll('.copy-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const text = btn.getAttribute('data-copy');
    navigator.clipboard.writeText(text).then(() => {
      btn.classList.add('copied');
      const svg = btn.querySelector('svg');
      const orig = svg.innerHTML;
      svg.innerHTML = '<polyline points="20 6 9 17 4 12" stroke="currentColor" stroke-width="2" fill="none"/>';
      setTimeout(() => {
        btn.classList.remove('copied');
        svg.innerHTML = orig;
      }, 1500);
    });
  });
});

// ==================== INTERACTIVE DEMOS ====================
(function initDemos() {
  const demoSection = document.getElementById('demo');
  if (!demoSection) return;

  // Tab switching
  demoSection.querySelectorAll('.demo-tab').forEach(tab => {
    tab.addEventListener('click', () => {
      demoSection.querySelectorAll('.demo-tab').forEach(t => t.classList.remove('active'));
      demoSection.querySelectorAll('.demo-panel').forEach(p => p.classList.remove('active'));
      tab.classList.add('active');
      const panel = demoSection.querySelector(`#${tab.dataset.panel}`);
      if (panel) panel.classList.add('active');
    });
  });

  // --- GCounter Demo ---
  const gcState = { a: { a: 0, b: 0 }, b: { a: 0, b: 0 } };

  function gcValue(node) {
    const s = gcState[node];
    return (s.a || 0) + (s.b || 0);
  }

  function gcUpdateUI() {
    const elA = document.getElementById('gc-node-a');
    const elB = document.getElementById('gc-node-b');
    if (!elA || !elB) return;
    elA.querySelector('.merge-node-value').textContent = gcValue('a');
    elB.querySelector('.merge-node-value').textContent = gcValue('b');
    elA.querySelector('.merge-node-detail').textContent = `{a: ${gcState.a.a}, b: ${gcState.a.b}}`;
    elB.querySelector('.merge-node-detail').textContent = `{a: ${gcState.b.a}, b: ${gcState.b.b}}`;
  }

  function gcLog(msg) {
    const log = document.getElementById('gc-log');
    if (!log) return;
    log.innerHTML += msg + '\n';
    log.scrollTop = log.scrollHeight;
  }

  document.getElementById('gc-inc-a')?.addEventListener('click', () => {
    gcState.a.a++;
    document.getElementById('gc-node-a')?.classList.add('active');
    setTimeout(() => document.getElementById('gc-node-a')?.classList.remove('active'), 300);
    gcLog(`<span class="log-teal">node-a</span>.increment()  // value = ${gcValue('a')}`);
    gcUpdateUI();
  });

  document.getElementById('gc-inc-b')?.addEventListener('click', () => {
    gcState.b.b++;
    document.getElementById('gc-node-b')?.classList.add('active');
    setTimeout(() => document.getElementById('gc-node-b')?.classList.remove('active'), 300);
    gcLog(`<span class="log-orange">node-b</span>.increment()  // value = ${gcValue('b')}`);
    gcUpdateUI();
  });

  document.getElementById('gc-merge')?.addEventListener('click', () => {
    // Merge: take max of each entry
    gcState.a.a = Math.max(gcState.a.a, gcState.b.a);
    gcState.a.b = Math.max(gcState.a.b, gcState.b.b);
    gcState.b.a = Math.max(gcState.a.a, gcState.b.a);
    gcState.b.b = Math.max(gcState.a.b, gcState.b.b);

    const arrow = document.getElementById('gc-arrow');
    arrow?.classList.add('visible');
    document.getElementById('gc-node-a')?.classList.add('merged');
    document.getElementById('gc-node-b')?.classList.add('merged');
    setTimeout(() => {
      arrow?.classList.remove('visible');
      document.getElementById('gc-node-a')?.classList.remove('merged');
      document.getElementById('gc-node-b')?.classList.remove('merged');
    }, 800);

    gcLog(`<span class="log-purple">merge!</span>  node-a = ${gcValue('a')}, node-b = ${gcValue('b')}  // <span class="log-teal">converged</span>`);
    gcUpdateUI();
  });

  document.getElementById('gc-reset')?.addEventListener('click', () => {
    gcState.a = { a: 0, b: 0 };
    gcState.b = { a: 0, b: 0 };
    const log = document.getElementById('gc-log');
    if (log) log.innerHTML = '';
    gcUpdateUI();
  });

  // --- ORSet Demo ---
  const orState = {
    a: new Map(), // tag -> element
    b: new Map(),
    nextTag: 1,
  };

  function orElements(node) {
    return [...new Set(orState[node].values())].sort();
  }

  function orUpdateUI() {
    const elA = document.getElementById('or-node-a');
    const elB = document.getElementById('or-node-b');
    if (!elA || !elB) return;
    const itemsA = orElements('a');
    const itemsB = orElements('b');
    elA.querySelector('.merge-node-value').textContent = `{${itemsA.join(', ')}}`;
    elB.querySelector('.merge-node-value').textContent = `{${itemsB.join(', ')}}`;
    elA.querySelector('.merge-node-detail').textContent = `${itemsA.length} elements`;
    elB.querySelector('.merge-node-detail').textContent = `${itemsB.length} elements`;
  }

  function orLog(msg) {
    const log = document.getElementById('or-log');
    if (!log) return;
    log.innerHTML += msg + '\n';
    log.scrollTop = log.scrollHeight;
  }

  const orItems = ['milk', 'eggs', 'bread', 'butter', 'cheese', 'apple', 'rice'];
  let orItemIdx = 0;

  document.getElementById('or-add-a')?.addEventListener('click', () => {
    const item = orItems[orItemIdx % orItems.length]; orItemIdx++;
    const tag = orState.nextTag++;
    orState.a.set(tag, item);
    document.getElementById('or-node-a')?.classList.add('active');
    setTimeout(() => document.getElementById('or-node-a')?.classList.remove('active'), 300);
    orLog(`<span class="log-teal">node-a</span>.add("${item}")  // tag=${tag}`);
    orUpdateUI();
  });

  document.getElementById('or-add-b')?.addEventListener('click', () => {
    const item = orItems[orItemIdx % orItems.length]; orItemIdx++;
    const tag = orState.nextTag++;
    orState.b.set(tag, item);
    document.getElementById('or-node-b')?.classList.add('active');
    setTimeout(() => document.getElementById('or-node-b')?.classList.remove('active'), 300);
    orLog(`<span class="log-orange">node-b</span>.add("${item}")  // tag=${tag}`);
    orUpdateUI();
  });

  document.getElementById('or-remove-a')?.addEventListener('click', () => {
    const keys = [...orState.a.keys()];
    if (keys.length === 0) return;
    const lastKey = keys[keys.length - 1];
    const removed = orState.a.get(lastKey);
    orState.a.delete(lastKey);
    orLog(`<span class="log-teal">node-a</span>.remove("${removed}")  // tag=${lastKey}`);
    orUpdateUI();
  });

  document.getElementById('or-merge-set')?.addEventListener('click', () => {
    // Union of all tags
    const merged = new Map([...orState.a, ...orState.b]);
    orState.a = new Map(merged);
    orState.b = new Map(merged);

    document.getElementById('or-node-a')?.classList.add('merged');
    document.getElementById('or-node-b')?.classList.add('merged');
    setTimeout(() => {
      document.getElementById('or-node-a')?.classList.remove('merged');
      document.getElementById('or-node-b')?.classList.remove('merged');
    }, 800);

    orLog(`<span class="log-purple">merge!</span>  both = {${orElements('a').join(', ')}}  // <span class="log-teal">converged</span>`);
    orUpdateUI();
  });

  document.getElementById('or-reset-set')?.addEventListener('click', () => {
    orState.a = new Map(); orState.b = new Map(); orState.nextTag = 1; orItemIdx = 0;
    const log = document.getElementById('or-log');
    if (log) log.innerHTML = '';
    orUpdateUI();
  });

  // --- LWWRegister Demo ---
  const lwwState = {
    a: { value: '""', ts: 0 },
    b: { value: '""', ts: 0 },
    clock: 0,
  };
  const lwwValues = ['"Alice"', '"Bob"', '"Charlie"', '"Diana"', '"Eve"', '"Frank"'];
  let lwwIdx = 0;

  function lwwUpdateUI() {
    const elA = document.getElementById('lww-node-a');
    const elB = document.getElementById('lww-node-b');
    if (!elA || !elB) return;
    elA.querySelector('.merge-node-value').textContent = lwwState.a.value;
    elB.querySelector('.merge-node-value').textContent = lwwState.b.value;
    elA.querySelector('.merge-node-detail').textContent = `ts=${lwwState.a.ts}`;
    elB.querySelector('.merge-node-detail').textContent = `ts=${lwwState.b.ts}`;
  }

  function lwwLog(msg) {
    const log = document.getElementById('lww-log');
    if (!log) return;
    log.innerHTML += msg + '\n';
    log.scrollTop = log.scrollHeight;
  }

  document.getElementById('lww-set-a')?.addEventListener('click', () => {
    lwwState.clock++;
    const val = lwwValues[lwwIdx % lwwValues.length]; lwwIdx++;
    lwwState.a = { value: val, ts: lwwState.clock };
    document.getElementById('lww-node-a')?.classList.add('active');
    setTimeout(() => document.getElementById('lww-node-a')?.classList.remove('active'), 300);
    lwwLog(`<span class="log-teal">node-a</span>.set(${val})  // ts=${lwwState.clock}`);
    lwwUpdateUI();
  });

  document.getElementById('lww-set-b')?.addEventListener('click', () => {
    lwwState.clock++;
    const val = lwwValues[lwwIdx % lwwValues.length]; lwwIdx++;
    lwwState.b = { value: val, ts: lwwState.clock };
    document.getElementById('lww-node-b')?.classList.add('active');
    setTimeout(() => document.getElementById('lww-node-b')?.classList.remove('active'), 300);
    lwwLog(`<span class="log-orange">node-b</span>.set(${val})  // ts=${lwwState.clock}`);
    lwwUpdateUI();
  });

  document.getElementById('lww-merge-reg')?.addEventListener('click', () => {
    // LWW: highest timestamp wins
    const winner = lwwState.a.ts >= lwwState.b.ts ? lwwState.a : lwwState.b;
    lwwState.a = { ...winner };
    lwwState.b = { ...winner };

    document.getElementById('lww-node-a')?.classList.add('merged');
    document.getElementById('lww-node-b')?.classList.add('merged');
    setTimeout(() => {
      document.getElementById('lww-node-a')?.classList.remove('merged');
      document.getElementById('lww-node-b')?.classList.remove('merged');
    }, 800);

    lwwLog(`<span class="log-purple">merge!</span>  winner = ${winner.value} (ts=${winner.ts})  // <span class="log-teal">last writer wins</span>`);
    lwwUpdateUI();
  });

  document.getElementById('lww-reset-reg')?.addEventListener('click', () => {
    lwwState.a = { value: '""', ts: 0 };
    lwwState.b = { value: '""', ts: 0 };
    lwwState.clock = 0; lwwIdx = 0;
    const log = document.getElementById('lww-log');
    if (log) log.innerHTML = '';
    lwwUpdateUI();
  });

  // Init all UIs
  gcUpdateUI();
  orUpdateUI();
  lwwUpdateUI();
})();
