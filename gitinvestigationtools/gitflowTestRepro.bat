mkdir server
cd server
git init
git flow init -d
git checkout --detach

cd ..
git clone server client

cd client
git flow init -d

git flow feature start BUG-100
git branch
git flow feature list
echo erste Zeile > read.me
git add *
git commit -m "erste Zeile"
echo zweite Zeile >> read.me
git stage *
git commit -m "zweite Zeile"
git flow feature finish BUG-100

git flow release start 1.0
echo dritte Zeile >> read.me
git stage *
git commit -m "dritte Zeile"
git flow release finish -F -p 1.0
git push origin master
pause

git push origin develop
git pull origin master
git pull origin develop


git flow feature start BUG-200
git branch
git flow feature list
echo vierte Zeile > read.me
git add *
git commit -m "vierte Zeile"
echo fünfte Zeile >> read.me
git stage *
git commit -m "fünfte Zeile"
git flow feature finish BUG-200


git flow release start 1.5
echo dritte Zeile >> read.me
git stage *
git commit -m "dritte Zeile"
git flow release finish -F -p 1.5
git push origin master
git push origin develop
git pull origin master
git pull origin develop




