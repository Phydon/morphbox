	open ~/main/morphbox/tests/input_test.csv --raw | lines | split column "," | transpose --header-row | str trim | save ~/main/morphbox/tests/nutable.csv
