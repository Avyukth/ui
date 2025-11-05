# Alert Component

Display important messages with title and description.

## Usage

```rust
use alert::*;

<Alert variant=AlertVariant::Default>
    <AlertTitle text="Heads up!" />
    <AlertDescription text="You can add components to your app." />
</Alert>

<Alert variant=AlertVariant::Destructive>
    <AlertTitle text="Error" />
    <AlertDescription text="Something went wrong." />
</Alert>
```

## JavaScript

```js
import { mount_alert } from './pkg/alert.js';
mount_alert('Heads up!', 'You can add components.', 'default');
mount_alert('Error', 'Something went wrong.', 'destructive');
```

## Variants

- `Default` - Informational alerts
- `Destructive` - Error/warning alerts

## Components

- `Alert` - Main container with role="alert"
- `AlertTitle` - Alert title
- `AlertDescription` - Alert description

Progress: 9/56 (16.1%)
