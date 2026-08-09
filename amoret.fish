complete -c amoret -f
complete -c amoret -n "string match -r '^amoret\s+\$' (commandline)" -a -- -d Options
complete -c amoret -s h -l help -d 'Print help'
complete -c amoret -s V -l version -d 'Print version'
complete -c amoret -s d -l daemon -d 'Run as a background daemon'
complete -c amoret -s v -l verbose -d 'Increase logging verbosity'
complete -c amoret -s q -l quiet -d 'Decrease logging verbosity'
complete -c amoret -s c -l config -r -k -a "(__fish_complete_suffix .toml)" -d 'Path to config file'
complete -c amoret -s p -l plugins -r -k -a "(__fish_complete_suffix .scm)" -d 'Path to steel script to be plugged'
