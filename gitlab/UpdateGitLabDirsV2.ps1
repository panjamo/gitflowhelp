#   Create App-Token on personal Gitlab page
#   Navigate to Gitlab Preferences -> Access tokens
#     https://gitlab.com/-/user_settings/personal_access_tokens --> $env:GITLAB_TOKEN
#     https://ctd-sv01.thinprint.de/-/profile/personal_access_tokens -> $env:CTD-VS01

Function createFolders ($gitlabhost, $company, $headers, $getprojectURLPart) {
    $page = 1
    $perPage = 100
    $allProjects = @()
    $id = (Invoke-RestMethod -Uri "$($gitlabhost)/api/v4/user" -Headers $headers).Id

    $allProjects = Invoke-RestMethod -Uri "$($gitlabhost)/api/v4/users/$id/projects?per_page=$perPage&archived=false" -Headers $headers
    do {
        $url = "$($gitlabhost)$($getprojectURLPart)/projects?include_subgroups=true&page=$page&per_page=$perPage&archived=false"
        Write-Host $url
        $response = Invoke-RestMethod -Uri $url -Headers $headers
        $allProjects += $response
        $page++
    } while ($response.Count -eq $perPage)

    $allProjects | ConvertTo-Json -Depth 3 | Out-File -FilePath "C:\tmp\allProjects.json"


    $WshShell = New-Object -comObject WScript.Shell

    $allProjects | % {
        $project = $_
        $project.path_with_namespace = $project.path_with_namespace -replace "cortado-group/thinprint/", ""
        # $project.path_with_namespace

        if ($project.path_with_namespace -match "^(.*)/([^/]*)$") {
            $RepoName = $matches[2]
            $MinusName = $RepoName + " (" + ($matches[1] -replace "/", "#") + ") " + $project.id + ($project.namespace.parent_id ? " in " + $project.namespace.parent_id : "")
            $MinusName = $MinusName -replace "cortado-group#thinprint#", ""
        }
        # $RepoName
        $cwd = (Get-Location).Path + "/"

        ($cwd + $project.path_with_namespace)

        if ($project.path_with_namespace) {
            if ((New-Item -ItemType directory ($cwd + $project.path_with_namespace) -ErrorAction SilentlyContinue) -or $true) {
                $fileNameClone = "__CLONE" + ".cmd"
                $fileNameDelete = "__REMOVE" + ".cmd"
                $filenameUrl = "__REMOTE" + ".url"
                $filenameIssue = "__NEW_ISSUE" + ".url"
                $filenameBug = "__NEW_BUG" + ".url"

                $content = @"
@echo off
if exist ".git" (
    echo Already cloned
    exit /b
)
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
echo $fileNameClone>> .git\info\exclude
echo $fileNameDelete>> .git\info\exclude
echo $filenameUrl>> .git\info\exclude
echo $filenameIssue>> .git\info\exclude
echo $filenameBug>> .git\info\exclude
echo diff.diff>> .git\info\exclude
"@
                $repoExcludeFilename = $cwd + $project.path_with_namespace + '\.git\info\exclude'
                if (Test-Path $repoExcludeFilename) {
                    $exclude = (Get-Content $repoExcludeFilename).Trim() | Where-Object { $_ -ne "" }
                    if ($exclude -notcontains $fileNameClone) { $exclude += $fileNameClone }
                    if ($exclude -notcontains $fileNameDelete) { $exclude += $fileNameDelete }
                    if ($exclude -notcontains $filenameUrl) { $exclude += $filenameUrl }
                    if ($exclude -notcontains $filenameIssue) { $exclude += $filenameIssue }
                    if ($exclude -notcontains $filenameBug) { $exclude += $filenameBug }
                    if ($exclude -notcontains "diff.diff") { $exclude += "diff.diff" }
                    $exclude | Set-Content $repoExcludeFilename -Encoding UTF8
                }

                $filePath = $cwd + ($project.path_with_namespace + "/" + $fileNameClone)
                [System.IO.File]::WriteAllText($filePath, $content, [System.Text.Encoding]::GetEncoding('iso-8859-1'))

                $content = @"
@echo off
echo Epmty %CD% completely, type [yes]:
set /p answer=""
if /I "%answer%" == "yes" (
    for %%F in (*.*) do if not "%%~nxF"=="$($fileNameClone)" if not "%%~nxF"=="$($fileNameDelete)" if not "%%~nxF"=="$($filenameUrl)" if not "%%~nxF"=="$($filenameBug)" if not "%%~nxF"=="$($filenameIssue)" del /F "%%F"
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

            switch ($RepoName) {
                "ezeep-blue" { $teamLable = "team::ezeepBlue" }
                "hub-team" { $teamLable = "team::hub" }
                "thinprint-engine-team" { $teamLable = "team::ThinPrintEngine" }
                Default { $teamLable = "team::ezeepPrintPath" }
            }

            $filePath = $cwd + ($project.path_with_namespace + "/" + $filenameIssue)
            $issueDescription = @"



/weight 1
/label ~"type::task"
/label ~"$teamLable"
/label ~"priority::medium"
/epic https://gitlab.com/groups/cortado-group/thinprint/-/epics/<<epic-id>>
"@
            [System.IO.File]::WriteAllText($filePath, ("[InternetShortcut]`r`nURL=" + $project.web_url + "/-/issues/new?issue[title]=newissue&issue[description]=" + [System.Web.HttpUtility]::UrlEncode($issueDescription)), [System.Text.Encoding]::GetEncoding('iso-8859-1'))

            $filePath = $cwd + ($project.path_with_namespace + "/" + $filenameBug)
            $issueDescription = @"
<details>
<summary>

#### Versions / Customer, click here to _expand._

</summary>

| Faulty Version | Fixed Version | Tested Version |
| --- | --- | ---- |
| n/a | n/a | n/a |

| Customer / Partner | Ticket Url |
| --- | --- |
| n/a | n/a |

</details>

/weight 1
/label ~"type::bug"
/label ~"$teamLable"
/label ~"priority::medium"
"@
            [System.IO.File]::WriteAllText($filePath, ("[InternetShortcut]`r`nURL=" + $project.web_url + "/-/issues/new?issue[title]=newissue&issue[description]=" + [System.Web.HttpUtility]::UrlEncode($issueDescription)), [System.Text.Encoding]::GetEncoding('iso-8859-1'))
        }
    }
}

# Remove all files named __NEW_ISSUE.url recursively from the current working directory
Get-ChildItem -Path (Get-Location).Path -Recurse -Filter __NEW_ISSUE.url | ForEach-Object {
    Remove-Item -Path $_.FullName -Force
}
Get-ChildItem -Path (Get-Location).Path -Filter *.lnk | ForEach-Object {
    Remove-Item -Path $_.FullName -Force
}

$repos = @()
$repos += (createFolders "https://ctd-sv01.thinprint.de" "" @{'PRIVATE-TOKEN' = $env:CTDVS01 } "/api/v4")| Sort-Object -Unique
$repos += (createFolders "https://gitlab.com" "" @{'PRIVATE-TOKEN' = $env:GITLAB_TOKEN } "/api/v4/groups/cortado-group") | Sort-Object -Unique
$repos | Export-Clixml -Path "repos.xml"
'"' + ($repos -join '", "') + '"' | Out-File -FilePath "repos.txt"

# Check for duplicates
# $repos = Import-Clixml -Path "repos.xml"
$uniqueRepos = $repos | Sort-Object  -Unique
if ($uniqueRepos.Count -ne $repos.Count) {
    Write-Host "Project that are active in 'gitlab.com' and 'ctd-sv01.thinprint.de':"
    $duplicateRepos = $repos | Group-Object | Where-Object { $_.Count -gt 1 }
    $duplicateRepos | ForEach-Object {
        $_.Group | ForEach-Object {
            Write-Host $_
        }
    }
} else {
    Write-Host "No duplicate entries found."
}
#

$uniqueRepos = $uniqueRepos | ForEach-Object { $_ -replace "/", "\" }

$parents = @()
$uniqueRepos | ForEach-Object {
    $folder = $_
    $parents += (Get-Item -Path $folder ).Parent
}
$uniqueParents = $parents | Sort-Object -Unique
$uniqueParents.Count

$uniqueParents | % {
    $subfolders = Get-ChildItem -Path $_ -Directory
    $subfolders | % {
        if (-not (Test-Path "$_\__NEW_ISSUE.url") -and (Get-ChildItem -Path $_ -File)) {
            Write-Host "Missing __NEW_ISSUE.url in $_"
        }
    }
}