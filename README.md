LTools — README

LTools are a set of small but useful free, open-source
productivity-enhancing command-line tools for UNIX, Linux and similar
open-source operating systems.  They embrace the UNIX philosophy in
that many of them are intended to be used as part of pipes that read
from standard input and write to standard output streams, and that
they combined so the output of one can be the input to another. Thus,
they are also interoperatable with existing UNIX/Linux commands,
whether they are built in or user-contributed.

Note that some of them name-clash with existing commands depending on
the host operating system. Therefore, order in the PATH environment
variable matters, and as a resolution mechanism, the LTools installer
also installs variants prefixed by "l" (e.g. spell(1) is also
available as — a soft-link, namely — lspell(1)).

Some LTools were originally started as simple programming exercises
when the author wanted to learn and play/experiment with the Rust
programming language, and in the month of February 2021, being between
two jobs, Jochen Leidner developed a bunch more and bundled them for
an initial 1.0.0 release.  It is hoped that folks find them useful,
and consider contributing, too.

· bintosrc
· detabify
· fortune
· library
· logmsg
· spell
· sregex
· tabify
· woc
· xmastree

LTools were Written by and are copyright ©2021 by Jochen L. Leidner.
All rights reserved; see LICENSE.txt for the license terms.
