$ErrorActionPreference = 'Stop'

Push-Location (Join-Path $PSScriptRoot '..')
try {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw 'Cargo was not found. Install Rust using Rustup and restart the terminal.'
    }

    # The project currently declares one target on a single line.
    $toolchain = Get-Content 'rust-toolchain.toml' -Raw
    if ($toolchain -notmatch '(?m)^targets\s*=\s*\[\s*"([^"]+)"\s*\]') {
        throw 'Expected one target in rust-toolchain.toml.'
    }
    $buildTarget = $Matches[1]

    $checks = @(
        ,@('fmt', '--check')
        ,@('clippy', '--all-targets', '--all-features', '--', '-D', 'warnings')
        ,@('build', '--target', $buildTarget)
        ,@('test')
    )
    foreach ($check in $checks) {
        Write-Host "Running: cargo $($check -join ' ')"
        & cargo @check
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }
    }
}
finally {
    Pop-Location
}
exit 0
