git init
git add *
IF /I .%1 == .-q (git commit -m "initial") ELSE (git commit)
attrib +H .git