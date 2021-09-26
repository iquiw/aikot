_aikot() {
	local i cur prev opts cmds
	cmds="add browse clip completion edit help init list pwgen show version"
	COMPREPLY=()
	cur=${COMP_WORDS[COMP_CWORD]}
	prev=${COMP_WORDS[COMP_CWORD-1]}
	if [ "${#COMP_WORDS[@]}" = "2" ]; then
		COMPREPLY=( $(compgen -W "$cmds" -- ${cur}) )
	fi
	case $prev in
	browse|clip|edit|show)	COMPREPLY=( $(compgen -W "$(${COMP_WORDS[0]} list)" -- ${cur}) )
				;;
	completion) COMPREPLY=( $(compgen -W "bash" -- ${cur}) )
		    ;;
	esac
}
complete -F _aikot aikot aikot.exe
