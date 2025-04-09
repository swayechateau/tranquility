# Check if WSL is enabled
if(!(Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux).State -eq 'Enabled') {
    # Enable WSL
    Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux -NoRestart

    # Enable Virtual Machine Platform
    Enable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform -NoRestart

    # Restart computer
    Restart-Computer -Confirm
}

# Install WSL 2 Linux kernel update package
$wslUpdatePath = "https://aka.ms/wsl2kernel"
$wslUpdatePackage = "$env:TEMP\wsl_update.msi"
Invoke-WebRequest -Uri $wslUpdatePath -OutFile $wslUpdatePackage -UseBasicParsing
Start-Process -Wait -FilePath msiexec.exe -ArgumentList "/i $wslUpdatePackage /qn"

# Set WSL version to 2
wsl --set-default-version 2

# Install your desired Linux distribution (replace 'Ubuntu' with the distribution you want)
$distro = "Ubuntu"
Invoke-WebRequest -Uri "https://aka.ms/wsl-$distro" -OutFile "$env:TEMP\$distro.appx" -UseBasicParsing
Add-AppxPackage -Path "$env:TEMP\$distro.appx"

# (Optional) Set the default WSL distribution (replace 'Ubuntu' with the distribution you installed)
wsl --set-default Ubuntu
