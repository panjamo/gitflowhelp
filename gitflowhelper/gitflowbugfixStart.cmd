@for /f %%i in ('git rev-parse --abbrev-ref HEAD') do set CURRENTBRANCH=%%i
@set /p ISSUE=Enter Issues for BUGFIX branch (on %CURRENTBRANCH%):
if %ISSUE%. == . exit 0
git flow bugfix start "%ISSUE%" "%CURRENTBRANCH%"
pause