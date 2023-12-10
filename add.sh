#!/usr/local/bin/zsh

if [ $# -ne 2 ]; then
	echo "Usage: $0 day name"
	exit 1
fi

/home/burnus/.cargo/bin/cargo new --lib day${1}_${2}
cp -r ../day00/benches day${1}_${2}/
cp -r ../day00/tests day${1}_${2}/
cp ../day00/src/lib.rs day${1}_${2}/src/
cd day${1}_${2}
echo '
[dev-dependencies]
criterion = "0.5.1"

[[bench]]
name = "test_benchmark"
harness = false' >> Cargo.toml
echo -e "use day${1}_${2}::run;\n$(cat benches/test_benchmark.rs)" > benches/test_benchmark.rs
# echo 'intcode_processor = { path = "../common/intcode_processor" }' >> Cargo.toml
aocf checkout $1
aocf brief > challenge.txt
aocf input > tests/challenge_input
/usr/local/bin/nvim src/lib.rs
