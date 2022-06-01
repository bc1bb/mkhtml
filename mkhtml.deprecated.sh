#!/usr/bin/env bash

echo "            _    _     _             _"
echo "  _ __ ___ | | _| |__ | |_ _ __ ___ | |"
echo " | '_ \` _ \| |/ / '_ \| __| '_ \` _ \| |"
echo " | | | | | |   <| | | | |_| | | | | | |"
echo " |_| |_| |_|_|\_\_| |_|\__|_| |_| |_|_|"
# http://patorjk.com/software/taag/
echo ""

if [[ -d "$(pwd)/pages" && -d "$(pwd)/parts" && -d "$(pwd)/builds" ]]; then
	PAGESDIR="$(pwd)/pages"
	PARTSDIR="$(pwd)/parts"
	BUILDSDIR="$(pwd)/builds"
# Check if mandatory folders exist, make variables if yes, create them if not.
fi

if [[ ! -d "$(pwd)/pages" || ! -d "$(pwd)/parts" || ! -d "$(pwd)/builds" ]]; then
	mkdir {pages,parts,builds} 2>/dev/null
	echo "pages, parts and builds folders were created by mkhtml in $(pwd)"
	exit 1
fi

if [ ! -w "$BUILDSDIR" ]; then
	echo "mkhtml (executed by $(whoami)) can't write in $BUILDSDIR"
	echo "Ask your system admin to change permissions or move this whole folder"
	exit 1
fi

echo "PAGESDIR: $PAGESDIR"
echo "PARTSDIR: $PARTSDIR"
echo "BUILDSDIR: $BUILDSDIR"

if [ -d "$(pwd)/src" ]; then
	SRCDIR="$(pwd)/src"
	echo "SRCDIR: $SRCDIR"
fi
# Check if there is a src/ folder
if [ -d "$(pwd)/static" ]; then
	STATDIR="$(pwd)/static"
	echo "STATDIR: $STATDIR"
fi
# Check if there is a static/ folder

echo ""
echo "Building files..."

# shellcheck disable=SC2164
cd "$PAGESDIR"
find . -type d -links 2 -exec mkdir -p "$BUILDSDIR/{}" \;

# https://stackoverflow.com/a/54563899
find . -type f -print0 | while IFS= read -r -d $'\0' file; do
	filename=$(echo "$file" | sed 's/.\///')
	newloc="$BUILDSDIR/$filename"
	touch $newloc 2>/dev/null
	echo "Building $file"
	echo "<!-- Built with mkhtml (https://tildegit.org/jusdepatate/mkhtml) -->" > "$newloc"
	cat {"$PARTSDIR/header.html","$PAGESDIR/$filename","$PARTSDIR/footer.html"} >> "$newloc"
	# build files using PARTS and PAGE
done

if [ -n "$SRCDIR" ]; then
	echo "Copying SRCDIR to BUILDSDIR"
	mkdir "$BUILDSDIR/src" 2>/dev/null
	cp -r $SRCDIR/* $BUILDSDIR/src
fi

if [ -n "$STATDIR" ]; then
	echo "Copying STATDIR to BUILDSDIR"
	cp -r $STATDIR/* $BUILDSDIR
fi

# shellcheck disable=SC2164
# shellcheck disable=SC2103
cd - > /dev/null

echo ""
echo "Looks like all files were built"
echo ""

echo "Please report any error to https://github.com/jusdepatate/mkhtml"
