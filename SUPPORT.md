# Support

## Questions about using flags-2-env

Open a discussion or an issue on the canonical repository:

- <https://github.com/ORESoftware/flags-2-env/issues>

Include your `.cli-flags.toml`, the argv you passed, the client you are calling
from, and the map you got back versus the one you expected. The CLI prints the
map that every client is expected to produce, which makes it a fast way to tell
a contract problem from a binding problem:

```console
$ flags2env --port 8181 --debug=t
{"PORT":"8181","DEBUG":"true"}
```

## "Which language do I install?"

Each client has a README under `clients/<lang>/` in the canonical repository,
and each has a runnable consumer fixture in
[flags-2-env-test](https://github.com/flags-2-env-test) that shows a complete
working project — manifest, flag contract, Dockerfile, and a program that prints
the resolved environment.

## This organization

Issues opened here will be closed with a pointer to
`ORESoftware/flags-2-env`. It is not a mirror and it does not track the library.
