<h1 align=center> ~ amoret </h1>

<p>
  simple Discord RPC client written on rust. 
</p>
<h1></h1>

basic usecase looks like:
```Sh
~: $ amoret --confg /path/to/config.toml --daemonize
# example config in repo's root
```

beside serialised with `serde` toml config `amoret` also could be expanded with `steel` (LISP dialect) with `--plugins` flag: 
basic usecase looks like:
```Sh
~: $ amoret --plugins /path/to/plugins.scm
# my plugin example isn't worky, idk how lisp gonna work bc im nix person
```

<h1></h1>

<h3>installing</h3>

at this moment `amoret` distributed as nix-only package. plans for ci/cd with binaries exist.
you could always:
```Sh
git clone https://github.com/s0me1newithhand7s/amoret.git
cd amoret/
cargo build .
```
and then use as is!
