# NOTE: THIS PROJECT IS ON VERY EARLY STAGE


(mirrored on [GitHub][GitHub mirror])

[![builds.sr.ht status](https://builds.sr.ht/~mcf/cproc.svg)](https://builds.sr.ht/~mcf/cproc)

`cproc` is a [C11] compiler using [QBE] as a backend. It is released
under the [ISC] license.

Several GNU C [extensions] are also implemented.

There is still much to do, but it currently implements most of the
language and is capable of [building software] including itself, mcpp,
gcc 4.7, binutils, and more.

It was inspired by several other small C compilers including [8cc],
[c], [lacc], and [scc].

## Requirements

The compiler itself is written in standard C11 and can be built with
any conforming C11 compiler.

The POSIX driver depends on POSIX.1-2008 interfaces, and the `Makefile`
requires a POSIX-compatible make(1).

At runtime, you will need QBE, an assembler, and a linker for the
target system. Currently, my personal [QBE branch] is recommended, since
it may address some issues that have not yet made it upstream. Since
the preprocessor is not yet implemented, an external one is currently
required as well.

## Supported targets

This fork of cproc work with new qbe compat implementation, which named as qberust.
qberust has DAG and DSL(like LLVM's tablegen) based selection algorithm to implement retargetable instruction selection.
currently, there is no target is supported. and we are planning to support few targets first in qberust.

**Planned targets**
* Very basic extensions with x64 archtecture
* General ARM64 architecture (without any extensions)
* General x86 archtecture (without any extensions)

### Bootstrap

The `Makefile` includes several other targets that can be used for
bootstrapping. These targets require the ability to run the tools
specified in `config.h`.

- **`stage2`**: Build the compiler with the initial (`stage1`) output.
- **`stage3`**: Build the compiler with the `stage2` output.
- **`bootstrap`**: Build the `stage2` and `stage3` compilers, and verify
  that they are byte-wise identical.

## What's missing

- Digraph and trigraph sequences ([6.4.6p3] and [5.2.1.1], will not
  be implemented).
- Wide string literals ([#35]).
- Variable-length arrays ([#1]).
- `volatile`-qualified types ([#7]).
- `_Thread_local` storage-class specifier ([#5]).
- `long double` type ([#3]).
- Inline assembly ([#5]).
- Preprocessor ([#6]).
- Generation of position independent code (i.e. shared libraries,
  modules, PIEs).

## Mailing list

There is a mailing list at [~mcf/cproc@lists.sr.ht]. Feel free to
use it for general discussion, questions, patches, or bug reports
(if you don't have an sr.ht account).

## Issue tracker

Please report any issues to https://todo.sr.ht/~mcf/cproc.

## Contributing

Patches are greatly appreciated. Send them to the mailing list
(preferred), or as pull-requests on the [GitHub mirror].

[QBE]: https://c9x.me/compile/
[C11]: http://port70.net/~nsz/c/c11/n1570.html
[ISC]: https://git.sr.ht/~mcf/cproc/blob/master/LICENSE
[extensions]: https://man.sr.ht/~mcf/cproc/doc/extensions.md
[building software]: https://man.sr.ht/~mcf/cproc/doc/software.md
[8cc]: https://github.com/rui314/8cc
[c]: https://github.com/andrewchambers/c
[lacc]: https://github.com/larmel/lacc
[scc]: http://www.simple-cc.org/
[QBE branch]: https://git.sr.ht/~mcf/qbe
[5.2.1.1]: http://port70.net/~nsz/c/c11/n1570.html#5.2.1.1
[6.4.6p3]: http://port70.net/~nsz/c/c11/n1570.html#6.4.6p3
[#1]: https://todo.sr.ht/~mcf/cproc/1
[#3]: https://todo.sr.ht/~mcf/cproc/3
[#5]: https://todo.sr.ht/~mcf/cproc/5
[#6]: https://todo.sr.ht/~mcf/cproc/6
[#7]: https://todo.sr.ht/~mcf/cproc/7
[#35]: https://todo.sr.ht/~mcf/cproc/35
[#44]: https://todo.sr.ht/~mcf/cproc/44
[~mcf/cproc@lists.sr.ht]: https://lists.sr.ht/~mcf/cproc
[GitHub mirror]: https://github.com/michaelforney/cproc
