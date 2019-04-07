@ECHO OFF
SET PATH=c:\Program Files\Git\usr\bin;%PATH%
SETLOCAL ENABLEEXTENSIONS
IF ERRORLEVEL 1 ECHO Unable to enable extensions

SET HELP=false
SET MODE=CLONE
:Loop
    set LOOPVAR=%~1
    if "%LOOPVAR%" == "" GOTO LeaveOptionLoop
    if %LOOPVAR:~0,2%. == -h.  ( set HELP=true)
    if %LOOPVAR:~0,2%. == -d.  ( set MODE=DELETE)
    if %LOOPVAR:~0,2%. == -r.  ( set MODE=UPDATENUGET)
    if %LOOPVAR:~0,2%. == -f.  ( pushd  %LOOPVAR:~2,100%)
    SHIFT
GOTO Loop
:LeaveOptionLoop

    REM if %LOOPVAR:~0,2%. == -c.  ( set COMMAND=%LOOPVAR:~2,100%)


if %HELP% == true (
    echo gg: get git repo
    echo    -h help
    echo    -d delete dir
    echo    -r restore nuget packages, and update ThinPrint.MSBuild 
    echo       and ThinPrint.MSBuild.mkversiov3 package
    echo    "-f..." startup folder
    echo.
    echo REMARKS:
    echo     These environment variables can be defined to configure gg.cmd
    echo.
    echo     This command executed as last command
    echo     default: COMMAND_LINE_TOOL="C:\Program Files\Git\git-bash.exe"
    echo.
    echo     Define GIT Repo Base Folders and their git prefix   
    echo     default: REPO_PREFIX=GIT#git@ctd-sv01.thinprint.de: HTTPS#https://ctd-sv01.thinprint.de/
    echo.
    echo     Defined Addition GIT init command
    echo     default: GIT_FLOW_INIT=call gitflowinit.cmd
    exit /b
)

if NOT DEFINED REPO_PREFIX (
    SET REPO_PREFIX=^
        GIT#git@ctd-sv01.thinprint.de:^
        HTTPS#https://ctd-sv01.thinprint.de/
) 
if NOT DEFINED COMMAND_LINE_TOOL (
    SET COMMAND_LINE_TOOL="C:\Program Files\Git\git-bash.exe"
) 
if NOT DEFINED GIT_FLOW_INIT (
    SET GIT_FLOW_INIT=call gitflowinit.cmd
) 
REM https://de.wikibooks.org/wiki/Batch-Programmierung:_Erweiterungen_unter_Windows_NT
For %%A in ("%CD%") do (
    set modulename=%%~nA%%~xA
    set pardirname=%%~pA
    )
For %%B in ("%pardirname:~0,-1%") do (
    set groupname=%%~nB%%~xB
    set pardirname=%%~pB
    )
For %%C in ("%pardirname:~0,-1%") do (
    set rootname=%%~nC%%~xC
    )

IF /I "%MODE%" == "UPDATENUGET" (
    find . -name "*.sln" | xargs -I {} nuget restore "{}"
    for /R "." %%f in (*.sln) do nuget.exe update -Id ThinPrint.MSBuild -Id ThinPrint.MSBuild.mkversiov3 "%%f"
    EXIT /B
)

if NOT %MODE% == DELETE GOTO CLONE
    echo Epmty %CD% completely, type [yes]:
    set /p answer=""
    echo %answer%
    if /I "%answer%" == "yes" (
        taskkill /IM "TortoiseGitProc.exe" /F
        rmdir . /s /q
        echo gg.cmd> "_git clone %groupname%---%modulename%.cmd"
        echo [InternetShortcut]>"gitlab %groupname%---%modulename%.url"
        echo URL=https://ctd-sv01.thinprint.de/%groupname%/%modulename%>>"gitlab %groupname%---%modulename%.url"
        EXIT
    )
    EXIT /B
:CLONE

FOR %%i IN (%REPO_PREFIX%) DO (
    FOR  /F "tokens=1-2 delims=#" %%a IN ('echo %%i') DO (
        echo checking %rootname% -- %%a (with %%b^)
        IF /I "%rootname%" == "%%a"  (
            set repo=%%b%groupname%/%modulename%.git
        )
    )
)

IF .%repo% == . (
    echo. 
    echo ERROR
    echo Could not determine repo link.
    echo Root dir - at 2 levels up - is: "%rootname%"
    echo Currently supported names are:
    FOR %%i IN (%REPO_PREFIX%) DO (
        FOR  /F "tokens=1-2 delims=#" %%a IN ('echo %%i') DO (
            echo   %%a
        )
    )
    pause
    exit
)

IF NOT EXIST .git (

    ECHO cloning %repo% ...
    del "_git clone %groupname%---%modulename%.cmd"
    del "gitlab %groupname%---%modulename%.url"
    git clone %repo% .
    echo gg -d> _removeall--%groupname%---%modulename%.cmd
    echo _removeall--%groupname%---%modulename%.cmd>> .git\info\exclude

    echo [InternetShortcut]>"gitlab %groupname%---%modulename%.url"
    echo URL=https://ctd-sv01.thinprint.de/%groupname%/%modulename%>>"gitlab %groupname%---%modulename%.url"
    echo gitlab %groupname%---%modulename%.url>> .git\info\exclude

    REM git checkout develop
    REM if ERRORLEVEL 1 (
    REM     PAUSE
    REM     EXIT /b
    REM )
    REM git checkout develop
    start /min git graph
    git submodule update --init --recursive
    git trackall
    git storeDevCorrespondingSupportBranch
    %GIT_FLOW_INIT%
    start "GIT %groupname%---%modulename%" %COMMAND_LINE_TOOL%
) ELSE (
    git fetch --prune
    echo gg -d> _removeall--%groupname%---%modulename%.cmd
    echo _removeall--%groupname%---%modulename%.cmd>> .git\info\exclude

    echo [InternetShortcut]>"gitlab %groupname%---%modulename%.url"
    echo URL=https://ctd-sv01.thinprint.de/%groupname%/%modulename%>>"gitlab %groupname%---%modulename%.url"
    echo gitlab %groupname%---%modulename%.url>> .git\info\exclude
    start "GIT %groupname%---%modulename%" %COMMAND_LINE_TOOL%
    start /min git graph
)
EXIT /b
