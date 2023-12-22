#!/bin/sh
aseqdump -p "Launchkey Mini MK3" | \
while IFS=" ," read src ev1 ev2 ch label1 data1 label2 data2 rest; do
    case "$ev1 $ev2 $data1" in
        "Note on 53" ) xdotool key j ;;
        "Note on 55" ) xdotool key f ;;
	"Note on 48" ) xdotool key g ;;
	"Note on 60" ) xdotool key h ;;
    esac
done
