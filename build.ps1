# powershell script set a regular expression template
# to match the name of the file and run the executable

$filePath = $args[0]
$reg = "([A-Za-z0-9-]*).rs$"
rustc $args[0] --out-dir ./bin &
if ($args[0] -match $reg) {
    while (!(Test-Path -Path "./bin/$($matches[1]).exe")) {
        Start-Sleep -Milliseconds 100
    }
    & ./bin/$($matches[1]).exe
}