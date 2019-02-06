# t64write

# what?

A little tool that puts exactly one PRG file in a T64 file.

# why?

Most software for the Commodore 64 is stored in disk images: D64's.
However, most software didn't come on disk, but on tape.
To keep tape loading bearable, these programs were often only a single file.
Having a disk image with only a single file on it wastes a lot of space,
and this always felt bad to me.
I know that disk images are under 200K, so what do I care?
Well, I just do.

# how?

To create bla.t64 and put bla.prg in it:

`t64write bla.t64 bla.prg`

The Commodore 64 filename is `bla.prg` until the first `.`: `bla`.

# where?

* [Linux download](https://github.com/laamella-gad/t64write/releases/download/v1.0.0/t64write.gz)
* Windows download (Does not exist yet. Can someone compile this on Windows please?)


# words.

The [T64 format](http://unusedino.de/ec64/technical/formats/t64.html) is crap.
It has nothing to do with tapes, really.
It's a kind of zip file without compression: there is a directory (on a tape?!) and all kinds of meta information,
it has support for custom emulator formats,
where I would just like to see something like "filename, length, start address, data, filename, length, start address, data, repeat".
The format is too complex for what it is, and not too well specified,
so the loaders for it that exist have to be very creative to get the correct data from the wide range of badly written T64 files.

Also: this is my first [Rust](https://www.rust-lang.org/) program.
It's crap.
If you know how to improve it, submit a PR.