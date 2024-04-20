$current_path = Get-Location

$user_env = [System.Environment]::GetEnvironmentVariable("Path", "User")

if(-not ($user_env -split ";" | Where-Object {$_ -eq $current_path})){
    $new_path = "$user_env;$current_path"
    [System.Environment]::SetEnvironmentVariable("Path", $new_path, "User")

    Write-Host "Done."
}