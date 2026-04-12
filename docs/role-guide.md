# TilShell – Neue Rollen anlegen

## 1. Rolle in `roles.rs` definieren

Öffne `src/tools/roles.rs` und füge die neue Rolle zur Enum hinzu:

```rust
pub enum UserRole {
    Lynn,
    Bene,
    Guest,
    NeueRolle,  // <-- hier eintragen
}
```

---

## 2. Login in `session_handling.rs` anpassen

Hier wird entschieden, welcher Username welche Rolle bekommt.

**Schritt 1:** Den neuen Username aus der `.env` Datei laden:

```rust
let neue_rolle_user = env::var("SHELL_USER_NEUE").unwrap_or_else(|_| "neueruser".to_string());
```

**Schritt 2:** Den neuen User im `match`-Block abfangen:

```rust
u if u == neue_rolle_user => {
    println!("Welcome, {}.", u);
    return Ok(UserRole::NeueRolle);
}
```

**Schritt 3:** In der `.env` Datei den Username eintragen:

```
SHELL_USER_NEUE="neueruser"
```

---

## 3. Commands einschränken in `parser.rs`

Hier wird gesteuert, wer welche Commands ausführen darf.

**Beispiel – Command nur für eine Rolle erlauben:**

```rust
"mein_command" => {
    match role {
        UserRole::NeueRolle => mein_command(),
        _ => println!("Permission denied."),
    }
}
```

**Beispiel – Command für alle außer Guest:**

```rust
"mein_command" => {
    match role {
        UserRole::Guest => println!("Permission denied."),
        _ => mein_command(),
    }
}
```

**Beispiel – Command für mehrere Rollen:**

```rust
"mein_command" => {
    match role {
        UserRole::Lynn | UserRole::NeueRolle => mein_command(),
        _ => println!("Permission denied."),
    }
}
```

---

## 4. Help-Text anpassen in `regularcomms.rs`

In der `help()` Funktion steht der `match role` Block der bestimmt, welche Commands in der Hilfe angezeigt werden.

Einfach einen neuen Zweig für die neue Rolle hinzufügen:

```rust
UserRole::NeueRolle => {
    commands.insert("mein_command", "Beschreibung des Commands.");
    // weitere Commands hier eintragen
}
```

---

## Kurzübersicht

| Schritt | Datei | Was tun |
|--------|-------|---------|
| 1 | `roles.rs` | Rolle zur Enum hinzufügen |
| 2 | `session_handling.rs` | Username → Rolle zuweisen |
| 3 | `.env` | Username eintragen |
| 4 | `parser.rs` | Commands auf Rollen einschränken |
| 5 | `regularcomms.rs` | Help-Text für Rolle definieren |
