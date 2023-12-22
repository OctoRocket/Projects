#!/bin/sh
aseqdump -p "Launchkey Mini MK3" | \
while IFS=" ," read src ev1 ev2 ch label1 data1 label2 data2 rest; do
    case "$ev1 $ev2 $data1" in
        "Note on 48" ) xdotool key s;;
        "Note on 50" ) xdotool key d;;
        "Note on 51" ) xdotool key f;;
        "Note on 53" ) xdotool key g;;
        "Note on 55" ) xdotool key h;;
        "Note on 56" ) xdotool key j;;
        "Note on 58" ) xdotool key k;;
        "Note on 60" ) xdotool key l;;
    esac
done
