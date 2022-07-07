# create human readable input file
# -> deletes duplicates
open ~/main/morphbox/input/input_test.csv --raw | lines | split column "," | transpose --header-row | str trim | save ~/main/morphbox/tests/nutable.csv
