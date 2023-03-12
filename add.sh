#!/usr/local/bin/zsh

if [ $# -ne 2 ]; then
	echo "Usage: $0 day name"
	exit 1
fi

/home/burnus/.cargo/bin/cargo new --lib day${1}_${2}
cp -r ../day00/tests day${1}_${2}/
cp ../day00/src/lib.rs day${1}_${2}/src/
cd day${1}_${2}
aocf checkout $1
aocf brief > challenge.txt
aocf input > tests/challenge_input
/usr/local/bin/nvim src/lib.rs
