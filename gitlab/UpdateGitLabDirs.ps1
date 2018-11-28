Get-GitLabProject | ForEach-Object {
    $MinusName = $_.path_with_namespace -replace "/", "---"

    New-Item -ItemType directory $_.path_with_namespace
    $filePath = ($_.path_with_namespace + "/_git clone " + $MinusName + ".cmd")
    [System.IO.File]::WriteAllText($filePath,"gg.cmd",[System.Text.Encoding]::GetEncoding('iso-8859-1'))

    $WshShell = New-Object -comObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut([IO.Path]::GetFullPath("./"+$MinusName+".lnk"))
    $Shortcut.TargetPath = [IO.Path]::GetFullPath($_.path_with_namespace)
    #$Shortcut.Arguments = '%*'
    $Shortcut.Save()

    }