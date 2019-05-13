@echo off

for /f "tokens=*"  %%i in ('git stash push --include-untracked -m temporarySTASH') do set stashResult=%%i
echo %stashResult%
for /f %%i in ('git rev-list "--max-parents=0" HEAD') do set SHA1=%%i
git branch master origin/master
git branch master %SHA1%
git branch develop origin/develop
git branch develop %SHA1%
git fetch 
git flow init -d -f
git config "gitflow.prefix.versiontag"  Rel-
IF NOT ".%stashResult%" == ".No local changes to save" (
    git stash pop
)
echo *.rc diff=RC >> .git\info\attributes
echo *.rc2 diff=RC >> .git\info\attributes
echo *.man diff=MAN >> .git\info\attributes
REM "echo *.nuspec diff=NUSPEC >> .git\info\attributes"
echo pom*.xml diff=POM >> .git\info\attributes
echo AssemblyInfo.cs diff=ASSEMBLYINFO >> .git\info\attributes
echo AssemblyInfo.cpp diff=ASSEMBLYINFO >> .git\info\attributes

git config diff.NUSPEC.textconv "sed 's/<\(version\|copyright\)>.*<\/\(version\|copyright\)>//i'"
REM git config diff.POM.textconv "sed 's/<version>.*<\/version>//i'"
git config diff.MAN.textconv "sed 's/version\s*=\s*.[0123456789.]*.//i'"
git config diff.RC.textconv "sed 's/^\s*\(FILEVERSION\|PRODUCTVERSION\|VALUE \"FileDescription\",\|VALUE \"CompanyName\",\|VALUE \"LegalCopyright\",\|VALUE \"ProductVersion\",\|VALUE \"FileVersion\",\).*//i'"
git config diff.ASSEMBLYINFO.textconv "sed 's/\(.*assembly:\s*\(AssemblyCopyright\|AssemblyInformationalVersion\|AssemblyFileVersion\|AssemblyVersion\|AssemblyCompany\|AssemblyInformationalVersionAttribute\|AssemblyFileVersionAttribute\|AssemblyVersionAttribute\).*\)\|( internal test release[^)]*)//i'"