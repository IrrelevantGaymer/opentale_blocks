A proc macro that lets users generate different types of blocks containing a name and an index.  It also generates a static slice of all the defined blocks called BLOCKS.  The slice is of type `&'static [&'static dyn BlockType]`.

A struct must implement a trait called BlockTypeBuildable and BlockType (that are both accessible in the file) to be used in this macro.

This is a very unhygienic macro, and I might be able to refactor this to a declarative macro instead to make it more hygienic.