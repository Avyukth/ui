# Breadcrumb Component

Navigation breadcrumbs showing the current location.

## Usage

```rust
use breadcrumb::*;

<Breadcrumb>
    <BreadcrumbList>
        <BreadcrumbItem>
            <BreadcrumbLink text="Home" href=Some("/".to_string()) />
        </BreadcrumbItem>
        <BreadcrumbSeparator children=None />
        <BreadcrumbItem>
            <BreadcrumbLink text="Components" href=Some("/components".to_string()) />
        </BreadcrumbItem>
        <BreadcrumbSeparator children=None />
        <BreadcrumbItem>
            <BreadcrumbPage text="Current Page" />
        </BreadcrumbItem>
    </BreadcrumbList>
</Breadcrumb>
```

## JavaScript

```js
import { mount_breadcrumb_simple } from './pkg/breadcrumb.js';
mount_breadcrumb_simple();
```

## Components

- `Breadcrumb` - Container with aria-label="breadcrumb"
- `BreadcrumbList` - Ordered list
- `BreadcrumbItem` - Individual item
- `BreadcrumbLink` - Clickable link
- `BreadcrumbSeparator` - Separator (defaults to "/")
- `BreadcrumbPage` - Current page (not clickable)

Progress: 10/56 (17.9%)
