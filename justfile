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
	$PSV = $PSVersionTable.PSVersion | % {"$_" -split "\." }
	$psver = $PSV[0] + "." + $PSV[1]
	if ($PSV[2].Length -lt 4) {
		$psver += "." + $PSV[2] + " Core"
	} else {
		$psver += " Desktop"
	}
	echo "PowerShell $psver"

create day:
    cargo generate --path ./daily-template --name {{day}}

run day part:
    cargo run --package {{day}} --bin {{part}} -- --data-dir {{day}}/
