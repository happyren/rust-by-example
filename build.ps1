$reg = "([A-Za-z0-9-]*).rs$"
rustc $matches[1] --out-dir ./bin &
if ($matches[0] -match $reg) {
    while (!(Test-Path "./bin/$($matches[1])")) {
        Start-Sleep -Milliseconds 100
    }
    ./bin/$($matches[1])
}