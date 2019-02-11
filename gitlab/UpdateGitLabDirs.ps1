# Prerequisits taken from: https://confluence.thinprint.de/x/eBVhBg
# 1. install PSGitLab: 
#    Find-Module -Name PSGitLab | Install-Module
# 2. Create App-Token on personal Gitlab page
#    https://ctd-sv01.thinprint.de/profile/personal_access_tokens
# 3. Prepare PSGitLab to use this token:
#    Save-GitLabAPIConfiguration -Domain https://ctd-sv01.thinprint.de  -Token "***SecretUserTokenFromGitLab***"

$WshShell = New-Object -comObject WScript.Shell

Get-GitLabProject | ForEach-Object {
    $MinusName = $_.path_with_namespace -replace "/", "---"
    $cwd = (Get-Location).Path + "/"

    if (New-Item -ItemType directory ($cwd + $_.path_with_namespace) -ErrorAction SilentlyContinue   ) {
        $filePath = $cwd + ($_.path_with_namespace + "/_git clone " + $MinusName + ".cmd")
        [System.IO.File]::WriteAllText($filePath,"gg.cmd",[System.Text.Encoding]::GetEncoding('iso-8859-1'))
        $filePath = $cwd + ($_.path_with_namespace + "/gitlab " + $MinusName + ".url")
        [System.IO.File]::WriteAllText($filePath, ("[InternetShortcut]`r`nURL=" + $_.web_url), [System.Text.Encoding]::GetEncoding('iso-8859-1'))
        
    
        $Shortcut = $WshShell.CreateShortcut([IO.Path]::GetFullPath($cwd+$MinusName+".lnk"))
        $Shortcut.TargetPath = [IO.Path]::GetFullPath($cwd+$_.path_with_namespace)
        #$Shortcut.Arguments = '%*'
        $Shortcut.Save()    
    }
}