External
========

External is a library for unsafe cross process interaction.

It does not attempt to abstract across OS APIs, rather it just provides a convenient rustic API around the OS API.

Windows only for now. Maybe it'll gain other OS API support in the distant future.

Features
--------

Wrappers for Process handles, Thread handles, HWND handles, Windows hooks.

Read, Write, Alloc, Free and Query virtual memory with convenient API which abstracts over pointers.

Iterate over processes, threads and modules using the toolhelp snapshot API.

Contributing
------------

This library will probably never be feature complete as I add things when I need them.

Feature requests may be considered but don't expect any commitment if I don't personally need your particular needs.

Contributions may be accepted but will be held to a higher standard than my own.

Last but not least, this library will not respect SemVer as I experiment with things and I don't like big SemVer numbers.

Library
-------

Currently unfinished and thus not published.

I may consider it after this library has proven its worth in another side project of mine.
