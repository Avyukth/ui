# Label Component

Form label with proper semantics and styling.

## Usage

```rust
use label::Label;

<Label for_id="email" text="Email Address" />
<input id="email" type="email" />
```

## JavaScript

```js
import { mount_label } from './pkg/label.js';
mount_label('Email Address', 'email');
```
