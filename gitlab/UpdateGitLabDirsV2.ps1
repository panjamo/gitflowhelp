# 1. install PSGitLab:
#    Find-Module -Name PSGitLab | Install-Module
# 2. Create App-Token on personal Gitlab page
#    Navigate to Gitlab Preferences -> Access tokens
# 3. Prepare PSGitLab to use this token:
#    Save-GitLabAPIConfiguration -Domain https://ctd-sv01.thinprint.de  -Token "***SecretUserTokenFromGitLab***"
#    Save-GitLabAPIConfiguration -Domain https://gitlab.com  -Token "***SecretUserTokenFromGitLab***"

$serchPar=@{}
$objHash = @{}
Get-GitLabGroup | % { $_.full_path -split "/" | % {$serchPar[$_]=$true}}
foreach ($key in $serchPar.Keys) {
    Write-Host $key
    Get-GitLabGroup -Search $key | % { Get-GitLabProject -GroupId $_.Id -ErrorAction SilentlyContinue } | ForEach-Object {
        $objHash[$_.Id] = $_
    }
}
$objHash.count

$WshShell = New-Object -comObject WScript.Shell

foreach ($key in $objHash.Keys) {
    $project = $objHash[$key]
    $project.path_with_namespace

    if ($project.path_with_namespace -match "^(.*)/([^/]*)$") {
        $RepoName = $matches[2]
        $MinusName = $RepoName + " (" + ($matches[1] -replace "/", "#") + ")"
    }

    $cwd = (Get-Location).Path + "/"

    if ($project.path_with_namespace) {
        if ((New-Item -ItemType directory ($cwd + $project.path_with_namespace) -ErrorAction SilentlyContinue) -or $true) {
            $fileNameClone = "__git clone " + $RepoName + ".cmd"
            $fileNameDelete = "__remove " + $RepoName + ".cmd"
            $filenameUrl = "__gitlab" + $RepoName + ".url"

            $content = "@echo off && git clone --bare $($project.ssh_url_to_repo) .git`n" +
            "git config --unset core.bare`n" +
            "git branch --list`n" +
            "echo Enter branch name to checkout, [type branch name, {enter} for keep]:`n" +
            "set /p answer=`"`"`n" +
            "git checkout %answer%`n" +
            "git submodule update --init --recursive`n" +
            "for /f `"tokens=* delims=`" %%i in ('git branch -r') do git branch --track `"%%i`" `"%%i`"`n"

            $filePath = $cwd + ($project.path_with_namespace + "/" + $fileNameClone)
            [System.IO.File]::WriteAllText($filePath, $content, [System.Text.Encoding]::GetEncoding('iso-8859-1'))

            $content = "@echo off`n" +
            "echo Epmty %CD% completely, type [yes]:" + "`n" +
            'set /p answer=""' + "`n" +
            'echo %answer%' + "`n" +
            'if /I "%answer%" == "yes" (' + "`n" +
            'for %%F in (*.*) do if not "%%~nxF"=="' + $fileNameClone + '" if not "%%~nxF"=="' + $fileNameDelete + '" if not "%%~nxF"=="' + $filenameUrl + '" del /F "%%F"' + "`n" +
            'attrib -h -r .git && rd /S /Q .git`n' + "`n" +
            'for /D %%G in (*) do rd /S /Q "%%G"' + "`n" +
            ')'

            $filePath = $cwd + ($project.path_with_namespace + "/" + $fileNameDelete)
            [System.IO.File]::WriteAllText($filePath, $content, [System.Text.Encoding]::GetEncoding('iso-8859-1'))

            $Shortcut = $WshShell.CreateShortcut([IO.Path]::GetFullPath($cwd + $MinusName + ".lnk"))
            $Shortcut.TargetPath = [IO.Path]::GetFullPath($cwd + $project.path_with_namespace)
            $Shortcut.Save()
        }

        $filePath = $cwd + ($project.path_with_namespace + "/" + $filenameUrl)
        [System.IO.File]::WriteAllText($filePath, ("[InternetShortcut]`r`nURL=" + $project.web_url), [System.Text.Encoding]::GetEncoding('iso-8859-1'))
    }
}