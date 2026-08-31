<h1 align=center> ~ amoret </h1>

<p>
  simple Discord RPC client written on rust. 
</p>
<h1></h1>

basic usecase looks like:

```Sh
~: $ amoret --confg /path/to/config.<toml|scm> --daemonize
# example config in repo's root
~: $ amoret --reload # used to stop previous session and start new
```

you can check config via `--validate` flag:
```Sh
~: $ amoret --validate --config /path/to/config.<toml|scm>
# --validate will fail if used with --daemonize or --reload
```

<h1></h1>

<h3>installing</h3>

under release you have **singed** binaries build via **GHA**.
choose your platform, install tarball, inpack it and use!

