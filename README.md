# Installation

## Sourcetree integration

### Create a hardlink to the sourcetree customs commands file
* navigate to *sourcetree* folder of this repo on your disk after cloning
* delete _%USERPROFILE%\AppData\Local\Atlassian\SourceTree_
* run following command with administrative rights
```
mklink %USERPROFILE%\AppData\Local\Atlassian\SourceTree  <rootpath>\customactions.xml
```
## Additional git commands

### Extend your global .gitconfig
```
git config --global include.path <rootpath>/gitflowhelp/githelper/.gitconfigalias
```

### include to path
* Add <rootpth>\gitflowhelper\gitflowhelper\ to your PATH environment variable **OR**
* set hardlink to your tools folder whitch is already included in your PATH environment variable

## Powershell (UpdateGitLabDirs.ps1) 

### Step 1
* open the Powershell in **elevated mode**
* copy the following line in the console and press enter

```powershell
Find-Module -Name PSGitLab | Install-Module
```

### Step 2
* Navigate to Gitlab Preferences -> Access tokens
* Create a *Personal Access Token* with _"api Access the authenticated user's API"_ checkbox checked

### Step 3
* open the Powershell in **elevated mode**
* copy the following line in the console and press enter

```powershell
Save-GitLabAPIConfiguration -Domain https://ctd-sv01.XXXXXXXXX.de -Token "<insert token here>"
```

* Create folder "GIT" on your disk
* start UpdateGitLabDirs.ps1 in this folder als working directory

## Configure _git clone  \<projectdir\>---\<projectname\>.cmd

* add <rootdir>\gitflowhelper\githelper to your path variable, or make <rootdir>\gitflowhelper\githelper\gg.cmd somehow accessable bei PATH, e.g. to make an hard-link run following command (elevated mode):

```make symbolic link
mklink c:\tools.cmd <rootdir>\gitflowhelper\githelper\gg.cmd
```