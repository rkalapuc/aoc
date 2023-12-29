# cross platform shebang
shebang := if os() == 'windows' {
    'powershell.exe'
} else {
    '/usr/bin/env pwsh'
}

# set shell for non-Windows OS
set shell := ["powershell", "-c"]

# set shell for Windows OS
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# if you have PowerShell Core installed and want to use it
# use `pwsh.exe` instead of `powershell.exe`

default:
    just --list

shebang:
    #!{{shebang}}
    $psv = $PSVersionTable.PSVersion | % {"$_" -split "\." }
    $psVersion = $psv[0] + "." + $psv[1]
    if ($psv[2].Length -lt 4) {
        $psVersion += "." + $psv[2] + " Core"
    } else {
        $psVersion += " Desktop"
    }
    echo "PowerShell $psVersion"

create year day:
    cargo generate --path ./daily-template --name aoc{{year}}-day-{{day}}
    move aoc{{year}}-day-{{day}} {{year}}/day-{{day}}

run year day part:
    cargo run --package aoc{{year}}-day-{{day}} --bin part0{{part}} -- --data-dir {{year}}/day-{{day}}/
