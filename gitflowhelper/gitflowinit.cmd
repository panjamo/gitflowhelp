@echo off

IF %1. == . GOTO init

git clone "%1"
FOR /F "tokens=1-2 delims==/" %%I IN ("%1") DO ( set GitFile=%%J )
FOR /F "tokens=1-2 delims==." %%I IN ("%GitFile%") DO (set ProjectName=%%I)
pushd %ProjectName%
echo Run git flow init on: %CD%
pause

:init
@REM for /f %%i in ('git rev-parse --abbrev-ref HEAD') do set CURRENTBRANCH=%%i
git stash
for /f %%i in ('git rev-list "--max-parents=0" HEAD') do set SHA1=%%i
git branch master origin/master
git branch master %SHA1%
git branch develop origin/develop
git branch develop %SHA1%
git fetch
git flow init -d -f
git config "gitflow.prefix.versiontag"  Rel-
@REM git checkout "%CURRENTBRANCH%"
git stash apply
echo *.rc diff=RC >> .git\info\attributes
echo *.rc2 diff=RC >> .git\info\attributes
echo *.man diff=MAN >> .git\info\attributes
echo pom*.xml diff=POM >> .git\info\attributes
echo AssemblyInfo.cs diff=ASSEMBLYINFO >> .git\info\attributes
echo AssemblyInfo.cpp diff=ASSEMBLYINFO >> .git\info\attributes

git config diff.POM.textconv "sed 's/<version>.*<\/version>//i'"
git config diff.MAN.textconv "sed 's/version\s*=\s*.[0123456789.]*.//i'"
git config diff.RC.textconv "sed 's/^\s*\(FILEVERSION\|PRODUCTVERSION\|VALUE \"FileDescription\",\|VALUE \"CompanyName\",\|VALUE \"LegalCopyright\",\|VALUE \"ProductVersion\",\|VALUE \"FileVersion\",\).*//i'"
git config diff.ASSEMBLYINFO.textconv "sed 's/\(.*assembly:\s*\(AssemblyInformationalVersionAttribute\|AssemblyFileVersionAttribute\|AssemblyVersionAttribute\).*\)\|( internal test release[^)]*)//i'"

git ls-remote

@exit 0


