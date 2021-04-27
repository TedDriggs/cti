# stix

Typesafe library for working with STIX 2.0 objects.

This includes types to represent specific STIX objects, and proc-macros to auto-generate a STIX `Collection` that extends the standard.

## Defining STIX Types

STIX allows for additional fields, object types, and relationships beyond those defined in the standard.
The `stix` crate is designed to make working with a known set of those extensions type-safe using proc-macros.
When dealing with STIX that extends the standard, use these macros to define the set of extensions you want to process.
