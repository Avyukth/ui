# Skeleton Component

Animated loading placeholder.

## Usage

```rust
use skeleton::{Skeleton, SkeletonVariant};

// Rectangle
<Skeleton class="w-full h-12" />

// Circle
<Skeleton variant=SkeletonVariant::Circle class="w-12 h-12" />

// Text line
<Skeleton variant=SkeletonVariant::Text class="w-3/4" />
```

## JavaScript

```js
import { mount_skeleton, mount_skeleton_circle } from './pkg/skeleton.js';
mount_skeleton('w-full', 'h-12');
mount_skeleton_circle('w-12');
```

Progress: 6/56 (10.7%)
