# Module 3: Dynamic Form Component in Dioxus

The Dynamic Form component is the heart of a headless CMS. It allows us to render a UI based on a database-stored JSON schema (`CMSSchema`).

## How it Works
1. **The Schema**: A `ContentType` defines fields like `ShortText`, `LongText`, `Number`, or `Boolean`.
2. **The Component**: Our `DynamicForm` iterates over the `fields` vector in the schema.
3. **Pattern Matching**: We use Rust's powerful `match` statement on the `FieldType` enum to determine which HTML input to render.
4. **Local State**: We use `use_signal(|| HashMap::<String, serde_json::Value>::new())` to store the form's current values.
5. **On-Submit**: When the form is submitted, we pass the final `HashMap` back to the parent component via an `EventHandler`.

## Why this is Production-Ready
- **Schema-Driven**: You can add new content types in the database without updating the frontend code.
- **Type Safety**: Using `serde_json::Value` allows us to store arbitrary field data while still being type-safe at the application level.
- **Tailwind Integration**: Each input is styled using Tailwind classes (`class: "border rounded p-2"`), providing a consistent UI.
