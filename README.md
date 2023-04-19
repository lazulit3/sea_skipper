# sea_skipper

`sea_skipper` is a library that helps with [`sea-orm`](https://docs.rs/sea-orm) usage in some specific use cases.

This library is experimental and may change drastically during early stages of development; `sea_skipper` is currently unstable and breaking changes are expected. Please expect nothing.

## Here Be Dragons

Alternatively, this could be described as a collection of misguided, sick experiments conducted to avoid some code that felt evil...
only to have built a much greater evil along the way. I don't know the answer yet, but I'm going to use it until I find out. ;)

In the current state of things (goofing around), I don't intend to:
- publish this in its current state
- worry about stability or backwards compatability

If you find something useful here that you intend to use, please let me know (in a discussion?) and I'll be happy to consider cleaning things up or maintaining things beyond my immediate purposes (toying around).

In all seriousness, I do believe there is merit to the problems (& attempts to solve) that led me to some of this, I just know this ain't it right here, right now. :)

## Example of Usage

`examples/axum_example` contains an example project using `sea_skipper` with an HTTP API built with ['axum'](https://docs.rs/axum/latest/axum/) and `sea-orm`.
