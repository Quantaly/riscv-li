#!/bin/bash
set -eux

git config --global user.name $user_name
git config --global user.email $user_email
git remote set-url origin https://${github_token}@github.com/${repository}

TEMPDIR=$(mktemp -d)
cp -r .git $TEMPDIR
cd $TEMPDIR
git checkout -B gh-pages
cp -Lr $OLDPWD/web/public/* .
ls -R
ls -R $OLDPWD/web
if [ ! -d pkg ]; then
    echo "pkg doesn't exist, fuck this shit"
    exit 1
fi
git add .
git commit -m "Dump"
git push -fu origin gh-pages
