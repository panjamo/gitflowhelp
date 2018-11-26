@ECHO OFF

if "%REPO_PREFIX%" == "" (
    SET REPO_PREFIX=^
        GIT#git@ctd-sv01.thinprint.de:
) 

@echo off
if NOT .%1 == .-d GOTO CLONE
    echo Epmty %CD% completely, type [yes]:
    set /p answer=""
    echo %answer%
    if .%answer% == .yes (
        pskill TortoiseGitProc.exe
        rmdir . /s /q
    )
    EXIT

:CLONE
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

    FOR %%i IN (%REPO_PREFIX%) DO (
        FOR  /F "tokens=1-2 delims=#" %%a IN ('echo %%i') DO (
            echo checking %rootname% -- %%a (with %%b^)
            IF "%rootname%" == "%%a"  (
                set repo=%%b%groupname%/%modulename%.git
            )
        )
    )
    ECHO cloning %repo% ...

    IF NOT EXIST .git (

        git clone "%repo%" .
        if ERRORLEVEL 1 (
            PAUSE
            EXIT /b
        )
        call c:\bin\cmdr.bat
        git checkout develop
        start /min git graph
        start /MIN git submodule update --init --recursive
        git trackall
        git storeDevCorrespondingSupportBranch
        call "c:\bin\gitflowinit.cmd"
    ) ELSE (

        call c:\bin\cmdr.bat "git fetch --tags && git mp"
        start /min git graph
    )
    EXIT /b
