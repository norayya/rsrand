$current_path = Get-Location

$user_env = [System.Environment]::GetEnvironmentVariable("Path", "User")

if($user_env -split ";" | Where-Object {$_ -eq $current_path}) {
    $p = $user_env -replace [regex]::Escape($current_path), ""
    $p = $p -replace ";;", ";"
    $p = $p.TrimStart(';').TrimEnd(';')
    [System.Environment]::SetEnvironmentVariable("Path", $p, "User")

    Write-Host "Done."
}