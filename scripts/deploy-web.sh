#!/bin/bash
set -eux

git config --global user.name $user_name
git config --global user.email $user_email
git remote set-url origin https://${github_token}@github.com/${repository}

npx gh-pages -d web/public
