# 1. install PSGitLab:
#    Find-Module -Name PSGitLab | Install-Module
# 2. Create App-Token on personal Gitlab page
#    Navigate to Gitlab Preferences -> Access tokens
# 3. Prepare PSGitLab to use this token:
#    Save-GitLabAPIConfiguration -Domain https://ctd-sv01.thinprint.de  -Token "***SecretUserTokenFromGitLab***"
#    Save-GitLabAPIConfiguration -Domain https://gitlab.com  -Token "***SecretUserTokenFromGitLab***"

$serchPar = @{}
$objHash = @{}
Get-GitLabGroup | % { $_.full_path -split "/" | % { $serchPar[$_] = $true } }
foreach ($key in $serchPar.Keys) {
    Write-Host $key
    Get-GitLabGroup -Search $key | % { Get-GitLabProject -GroupId $_.Id -ErrorAction SilentlyContinue } | ForEach-Object {
        if (($_.path_with_namespace).count -gt 0) {
            $objHash[$_.Id] = $_
        }
    }
}
$objHash.count
# $objHash.Values | select path_with_namespace,web_url,ssh_url_to_repo  | Out-GridView

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
            $fileNameClone = "__CLONE" + ".cmd"
            $fileNameDelete = "__REMOVE" + ".cmd"
            $filenameUrl = "__REMOTE" + ".url"

            $content = @"
            @echo off
            git clone --recursive $($project.ssh_url_to_repo) clone_tmp
            robocopy clone_tmp . /E /MOVE /NJH /NJS /NDL /NFL
            git config --global alias.trackbr "! git branch -r | awk '{print `$1}' | awk '{split(`$0,a,\""origin/\""); print a[2]}' | xargs -I branchName git branch --track branchName origin/branchName  2> /dev/null"
            git trackbr > nul
            git config --global --unset alias.trackbr
            git branch --list
            echo Enter branch name to checkout, [type branch name, {enter} for keep]:
            set /p answer=""
            git checkout %answer%
            git submodule update --init --recursive
            echo __CLONE.cmd>> .git\info\exclude
            echo __REMOVE.cmd>> .git\info\exclude
            echo __REMOTE.url>> .git\info\exclude
            echo diff.diff>> .git\info\exclude
"@

            $filePath = $cwd + ($project.path_with_namespace + "/" + $fileNameClone)
            [System.IO.File]::WriteAllText($filePath, $content, [System.Text.Encoding]::GetEncoding('iso-8859-1'))

            $content = @"
            @echo off
            echo Epmty %CD% completely, type [yes]:
            set /p answer=""
            if /I "%answer%" == "yes" (
                for %%F in (*.*) do if not "%%~nxF"=="$($fileNameClone)" if not "%%~nxF"=="$($fileNameDelete)" if not "%%~nxF"=="$($filenameUrl)" del /F "%%F"
                attrib -h -r .git && rd /S /Q .git
                for /D %%G in (*) do rd /S /Q "%%G"
            )
"@

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