#!/bin/bash
set -eux

git config --global user.name $user_name
git config --global user.email $user_email
git remote set-url origin https://${github_token}@github.com/${repository}

TEMPDIR=$(mktemp -d)
cp -r .git $TEMPDIR
cd $TEMPDIR
git checkout -B gh-pages
cp -LR $OLDPWD/web/public/* .
rm pkg/.gitignore pkg/package.json pkg/*.ts
git add .
git commit -m "Dump"
git push -fu origin gh-pages
