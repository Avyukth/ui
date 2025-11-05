# Card Component

Container with header, content, and footer sections.

## Usage

```rust
use card::*;

<Card>
    <CardHeader>
        <CardTitle text="Card Title" />
        <CardDescription text="Card description goes here" />
    </CardHeader>
    <CardContent>
        <p>"Your content here"</p>
    </CardContent>
    <CardFooter>
        <button>"Action"</button>
    </CardFooter>
</Card>
```

## JavaScript

```js
import { mount_card_simple } from './pkg/card.js';
mount_card_simple('Title', 'Description', 'Content');
```

## Components

- `Card` - Main container
- `CardHeader` - Header section
- `CardTitle` - Title text
- `CardDescription` - Description text
- `CardContent` - Content section
- `CardFooter` - Footer section

Progress: 8/56 (14.3%)
