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
git config --global include.path <rootpath>\gitflowhelper\githelper\.gitconfigalias
```

### include to path
* Add <rootpth>\gitflowhelper\gitflowhelper\ to yout PATH environment variable **OR**
* set hardlink to your tools folder whitch is already included in your PATH environment variable
