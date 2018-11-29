@ECHO OFF
if "%REPO_PREFIX%" == "" (
    SET REPO_PREFIX=^
        GIT#git@ctd-sv01.thinprint.de:^
        SSH#https://ctd-sv01.thinprint.de/
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
        echo gg.cmd > "_git clone %groupname%---%modulename%.cmd"
    )
    EXIT

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
        echo gg -d > _removeall--%groupname%---%modulename%.cmd
        echo _removeall--%groupname%---%modulename%.cmd >> .git\info\exclude
        git co develop
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
