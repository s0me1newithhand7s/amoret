set -l cmd amoret

complete -c $cmd -f

set -l no_opts "string match -r '^'$cmd'\s+\$?' (commandline)"

complete -c $cmd -n $no_opts -a -- -d Options

complete -c $cmd -s h -l help -d 'Print help'
complete -c $cmd -s V -l version -d 'Print version'
complete -c $cmd -s d -l daemon -d 'Run as a background daemon'
complete -c $cmd -s v -l verbose -d 'Increase logging verbosity'
complete -c $cmd -s q -l quiet -d 'Decrease logging verbosity'

complete -c $cmd -s c -l config -r -k -a "(__fish_complete_suffix .toml)" -d 'Path to config file'
complete -c $cmd -s p -l plugins -r -k -a "(__fish_complete_suffix .scm)" -d 'Path to steel script to be plugged'
