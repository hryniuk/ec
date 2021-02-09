# ec

The Educational Computer, Model 1 - project from the "Etudes for Programmers"
book.

`ec` is a computer simulator. Like an every standard computer program it
transforms data. As an input it accepts **ALF** (_absolute load file_; see
below) and **input stream**. It prints an **output stream** - the result of a
given program.

As I've moved to other projects, this one has been archived.
Implemented OP Codes can be found [here](https://github.com/hryniuk/ec/issues/1).

## Building

A default `Makefile`'s job builds a debug version and run both unit and
functional/system tests:

```shell
$ make
```

If you want to build it only, you can run:

```shell
$ make build
```

or just use `cargo`.

## Running

```shell
$ ./ec
Usage: ./ec [options]

Options:
    -q, --quiet         disable logs
    -v, --verbose       be verbose
    -t, --trace         be even more verbose - enables TRACE log level
    -f, --alf PATH      path to ALF
```

## ALF

ALF example:

```
00004004040000005400442E010020
END0040
```

**ALF** (_absolute load file_) is a text file that describes a content of **EC memory** before program
execution. It can be produced by the user, the **ecasm** (_assembler_) or the
**ecld** (loader) (both may be found in [EC Loader
repository](git@github.com:hryniuk/asm-ec.git)). It consist of **records**
explained below.

### Record

A single line of **ALF**, except the last one, is called a **record**. It
consists of:
* **checksum** (1st character)
* **sequence number** (characters 2-4)
* list of **entries**

**Checksum** is calculated by adding all the other hexadecimal digits and
ignoring the carryout.

**Sequence number** is a consecutive number, starting from **000**. As it uses
3 digits, the maximal value is `0xFFF`, hence single ALF may contain up to `4096`
records.

> All numbers present in a **record** are hexadecimal.

For example, in the **record** presented above:

```
0 000 4004040000005400442E010020
```

**checksum** equals `0` (cause sum of the rest of digits equals `48 = 0x30`) and
**sequence number** equals `000`. The last part is a list of **entries**.

The last **record** of **ALF** always starts with "END" string and contains 4
digits of **start address** (`0x40` in example).

#### Entries

Single **entry** describes a continuous chunk of **EC's memory**. It includes
three things:

* **count** - one digit; how many character positions in memory are to be filled by
  the data
* **address** - 4 digits; hexadecimal start address for the data in memory
* **data** - 2 digits for each characters to be filled (so 2 * **count** in
  overall).

There are two entries in the sample **record**, each with **count** = `4`:

```
4 0040 40 00 00 05
4 0044 2E 01 00 20
```

So this record translates to the following memory content:

```
address:    ... 0x40 0x41 0x42 0x43 0x44 0x45 0x46 0x47 ...
character:  ... 0x40 0x00 0x00 0x05 0x2E 0x01 0x00 0x20 ...
```


## License

MIT License

Copyright (c) 2017-2019 Łukasz Hryniuk

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
