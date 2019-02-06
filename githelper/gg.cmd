@ECHO OFF
SETLOCAL ENABLEEXTENSIONS
IF ERRORLEVEL 1 ECHO Unable to enable extensions
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

if NOT .%1 == .-d GOTO CLONE
    echo Epmty %CD% completely, type [yes]:
    set /p answer=""
    echo %answer%
    if /I .%answer% == .yes (
        taskkill /IM "TortoiseGitProc.exe" /F
        rmdir . /s /q
        echo gg.cmd> "_git clone %groupname%---%modulename%.cmd"
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
    
    ECHO cloning %repo% ...

    IF NOT EXIST .git (
        del "_git clone %groupname%---%modulename%.cmd"
        git clone %repo% .
        echo gg -d> _removeall--%groupname%---%modulename%.cmd
        echo _removeall--%groupname%---%modulename%.cmd>> .git\info\exclude
        REM git checkout develop
        REM if ERRORLEVEL 1 (
        REM     PAUSE
        REM     EXIT /b
        REM )
        start "GIT %groupname%---%modulename%" %COMMAND_LINE_TOOL%
        REM git checkout develop
        start /min git graph
        start /MIN git submodule update --init --recursive
        git trackall
        git storeDevCorrespondingSupportBranch
        %GIT_FLOW_INIT%
    ) ELSE (
        git fetch --prune
        start "GIT %groupname%---%modulename%" %COMMAND_LINE_TOOL%
        start /min git graph
    )
    EXIT /b
