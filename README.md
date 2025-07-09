A declarative macro that lets users generate different types of items containing a name and an index that would reference a table of data.  It also generates a static slice of type `&'static [&'static dyn SliceTrait]` of all the defined items.

Items must have a type that implements the trait Buildable.

Here's an Example assuming Foo and Bar implement Buildsble and SliceTrait:

```
table!(SliceTrait, static SLICE_NAME = {
    A: Foo = ...;
    B: Bar = ...;
});
```

this would produce constants A and B, and produce a static slice called "SLICE_NAME" with elements &A and &B.