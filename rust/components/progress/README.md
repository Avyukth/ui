# Progress Component

Visual progress indicator.

## Usage

```rust
use progress::Progress;

<Progress value=60.0 />
<Progress value=30.0 max=50.0 />
```

## JavaScript

```js
import { mount_progress } from './pkg/progress.js';
mount_progress(60.0);
```

Features:
- Percentage-based progress
- Smooth transitions
- Customizable max value

Progress: 7/56 (12.5%)
