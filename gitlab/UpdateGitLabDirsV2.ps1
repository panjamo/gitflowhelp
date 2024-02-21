# 1. Create App-Token on personal Gitlab page
#    Navigate to Gitlab Preferences -> Access tokens
# 2. Prepare PSGitLab to use this token:
#    Add environment variable "GITLAB" with value of the token
$headers = @{
    'PRIVATE-TOKEN' = $env:GITLAB
}
$page = 1
$perPage = 100
$allProjects = @()
$id = (Invoke-RestMethod -Uri 'https://gitlab.com/api/v4/user' -Headers $headers).Id
$allProjects = Invoke-RestMethod -Uri "https://gitlab.com/api/v4/users/$id/projects?per_page=$perPage" -Headers $headers
do {
    $url = "https://gitlab.com/api/v4/groups/cortado-group/projects?include_subgroups=true&page=$page&per_page=$perPage"
    Write-Host $url
    $response = Invoke-RestMethod -Uri $url -Headers $headers
    $allProjects += $response
    $page++
} while ($response.Count -eq $perPage)

$allProjects | ConvertTo-Json -Depth 3 | Out-File -FilePath "C:\tmp\allProjects.json"

$WshShell = New-Object -comObject WScript.Shell

$allProjects | % {
    $project = $_
    $project.path_with_namespace

    if ($project.path_with_namespace -match "^(.*)/([^/]*)$") {
        $RepoName = $matches[2]
        $MinusName = $RepoName + " (" + ($matches[1] -replace "/", "#") + ")"
        $MinusName = $MinusName -replace "cortado-group#thinprint#", ""
    }

    $cwd = (Get-Location).Path + "/"

    if ($project.path_with_namespace) {
        if ((New-Item -ItemType directory ($cwd + $project.path_with_namespace) -ErrorAction SilentlyContinue) -or $true) {
            $fileNameClone = "__CLONE" + ".cmd"
            $fileNameDelete = "__REMOVE" + ".cmd"
            $filenameUrl = "__REMOTE" + ".url"
            $filenameIssue = "__NEW_ISSUE" + ".url"

            $content = @"
            @echo off
            git clone --recursive $($project.ssh_url_to_repo) clone_tmp
            robocopy clone_tmp . /E /MOVE /NJH /NJS /NDL /NFL
            git config --global alias.trackbr "! git branch -r | awk '{print `$1}' | awk '{split(`$0,a,\""origin/\""); print a[2]}' | xargs -I branchName git branch --track branchName origin/branchName  2> /dev/null"
            git trackbr > nul
            git config --global --unset alias.trackbr
            git lbr 2> nul & git branch -v --sort=-committerdate
            echo Enter branch name to checkout, [type branch name, {enter} for keep]:
            set /p answer=""
            git checkout %answer%
            git submodule update --init --recursive
            echo __CLONE.cmd>> .git\info\exclude
            echo __REMOVE.cmd>> .git\info\exclude
            echo __REMOTE.url>> .git\info\exclude
            echo __NEW_ISSUE.url>> .git\info\exclude
            echo diff.diff>> .git\info\exclude
"@

            $filePath = $cwd + ($project.path_with_namespace + "/" + $fileNameClone)
            [System.IO.File]::WriteAllText($filePath, $content, [System.Text.Encoding]::GetEncoding('iso-8859-1'))

            $content = @"
            @echo off
            echo Epmty %CD% completely, type [yes]:
            set /p answer=""
            if /I "%answer%" == "yes" (
                for %%F in (*.*) do if not "%%~nxF"=="$($fileNameClone)" if not "%%~nxF"=="$($fileNameDelete)" if not "%%~nxF"=="$($filenameUrl)" if not "%%~nxF"=="$($filenameIssue)" del /F "%%F"
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

        $filePath = $cwd + ($project.path_with_namespace + "/" + $filenameIssue)
        [System.IO.File]::WriteAllText($filePath, ("[InternetShortcut]`r`nURL=" + $project.web_url + '/-/issues/new'), [System.Text.Encoding]::GetEncoding('iso-8859-1'))
    }
}