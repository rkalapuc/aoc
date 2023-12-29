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


export RUST_BACKTRACE := "1"

AOC_COOKIES_FILE := "aoc-cookies.txt"

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

get-input year day:
    #!{{shebang}}
    if (!(Test-Path {{AOC_COOKIES_FILE}})) {
      Write-Error -Message "File '{{AOC_COOKIES_FILE}}' does not exist!" -Category ResourceUnavailable
      exit 1
    }

    $dayNoZeros = "{{day}}" -replace '^0+'
    $url = "https://adventofcode.com/{{year}}/day/$dayNoZeros/input"

    $outputPath = "{{year}}/day-{{day}}/input.txt"
    $cookies = Get-Content -path {{AOC_COOKIES_FILE}}

    try {
        $webClient = New-Object System.Net.WebClient
        $webClient.Headers.Add("Cookie", $cookies)
        $webClient.DownloadFile($url, $outputPath)
        $webClient.Dispose()

        $fileContent = Get-Content -path $outputPath -Raw
        if ($fileContent.Trim() -eq "Puzzle inputs differ by user.  Please log in to get your puzzle input.") {
          Write-Host "Error: Failed to download file - check '{{AOC_COOKIES_FILE}}' contains valid AoC cookies"
          exit 1
        }

        Write-Host "Input file downloaded successfully to: $outputPath"
    } catch {
        Write-Host "Error: $_.Exception.Message"
        exit 1
    }

run year day part:
    cargo run --package aoc{{year}}-day-{{day}} --bin part0{{part}} -- --data-dir {{year}}/day-{{day}}/
