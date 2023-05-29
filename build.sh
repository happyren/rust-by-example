reg="([A-Za-z0-9-]*).rs$"
rustc $1 --out-dir ./bin &
[[ $1 =~ $reg ]] && until [ -f ./bin/${BASH_REMATCH[1]} ]
do
    sleep 0.1
done
./bin/${BASH_REMATCH[1]}