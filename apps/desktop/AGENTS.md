# Ownership

React owns presentation; Rust owns native lifecycle. Renderer imports the DesktopClient, never credentials, processes or raw native tools.

Follow root implementation-first, consolidated-verification rules. Keep pure logic separate from I/O; add abstractions only for actual consumers.
