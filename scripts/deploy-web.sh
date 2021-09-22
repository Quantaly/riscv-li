#!/bin/bash
set -eux

git config --global user.name $user_name
git config --global user.email $user_email
git remote set-url origin https://${github_token}@github.com/${repository}

TEMPDIR=$(mktemp -d)
cp -LR web/public $TEMPDIR
git checkout -b gh-pages
git rm -rf *
cp -R $TEMPDIR/public/* .
git add .
git commit -m "Dump"
git push
