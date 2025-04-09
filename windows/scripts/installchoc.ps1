# Run PowerShell as Administrator before executing this script

# Check if Chocolatey is already installed
$chocoPath = Get-Command choco.exe -ErrorAction SilentlyContinue

if ($chocoPath -eq $null) {
    # Install Chocolatey
    Set-ExecutionPolicy Bypass -Scope Process -Force
    iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

    # Verify installation
    $chocoPath = Get-Command choco.exe -ErrorAction SilentlyContinue
    if ($chocoPath -ne $null) {
        Write-Host "Chocolatey installed successfully."
    } else {
        Write-Host "Chocolatey installation failed."
    }
} else {
    Write-Host "Chocolatey is already installed."
}
