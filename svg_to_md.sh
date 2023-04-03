#!/bin/sh
# usage: svg_to_md.sh <svg file> <output file>

if [ $# -ne 2 ]; then
	echo "usage: svg_to_md.sh <input.svg> <output.md>"
	exit 1
fi

echo "<div>" > "$2"
tail +2 "$1" | sed '/^\s*$/d' >> "$2"
echo "</div>" >> "$2"
