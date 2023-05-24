# maxiv_status
Get quick information on the MAX-IV accelerators.

This will run off to the MAX-IV public status page, and grab some relevent info.  Perhaps this could be useful for updating a status bar on tmux/screen/etc?

## Tmux
Add the following to your tmux config file.
`bash
set -g status-right "#[fg=colour255,bold,bg=default]#(/PATH/TO/THE/EXECUTABLE/maxiv_status) "
`

