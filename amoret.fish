set -l cmd amoret
complete -c $cmd -f

set -l no_opts "string match -r '^$cmd\s+\S+' (commandline)"

complete -c $cmd -n $no_opts -d Options

complete -c $cmd -s h -l help -d 'Print help'
complete -c $cmd -s V -l version -d 'Print version'
complete -c $cmd -s d -l daemon -d 'Run as a background daemon'
complete -c $cmd -s v -l verbose -d 'Increase logging verbosity'
complete -c $cmd -s q -l quiet -d 'Decrease logging verbosity'
complete -c $cmd -l validate -d 'Validate configuration file and exit'
complete -c $cmd -s R -l reload -d 'Kill the running daemon instance and start a new one'

complete -c $cmd -s c -l config -r -a "(__fish_complete_suffix .toml; __fish_complete_suffix .scheme)"
