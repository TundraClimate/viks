# viks

vim-like key crate

## Usage

Parse string to `Key`:

```rs
use viks::Key;

let a_key = Key::new("a").unwrap();
let shift_a_key = Key::new("<s-a>").unwrap();
let shift_a_key_alt = Key::new("A").unwrap();

assert_eq!(shift_a_key, shift_a_key_alt);
```

---

Parse string to `Keymap`(wrapper for `Vec<Key>`):

```rs
use viks::Keymap;

let zz_map = Keymap::new("zz").unwrap();
let exit_map = Keymap::new("ZZ").unwrap();
let exit_map_alt = Keymap::new("<s-z><s-z>");

assert_eq!(exit_map, exit_map_alt);
```

avaliable ascii characters.

### Mapping

Special tags:

| string                  | key                   |
| ----------------------- | --------------------- |
| `<enter>` or `<cr>`     | KeyCode::Enter        |
| `<tab>`                 | KeyCode::Tab          |
| `<esc>`                 | KeyCode::Esc          |
| `<leader>` or `<space>` | KeyCode::Space        |
| `<bs>`                  | KeyCode::Backspace    |
| `<del>`                 | KeyCode::Delete       |
| `<lt>`                  | KeyCode::LessThanSign |

Modifier tags:

| string | key                  |
| ------ | -------------------- |
| <s-{}> | KeyModifier::Shift   |
| <a-{}> | KeyModifier::Alt     |
| <c-{}> | KeyModifier::Control |

## LICENSE

MIT
