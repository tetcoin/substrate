#!/usr/bin/env sh
#
# check if a pr is compatible with tetcoin companion pr or master if not
# available
#
# to override one that was just mentioned mark companion pr in the body of the
# tetcoin pr like
#
# tetcoin companion: tetcoin/tetcoin#567
#

set -e

github_api_tetcore_pull_url="https://api.github.com/repos/tetcoin/tetcore/pulls"
# use github api v3 in order to access the data without authentication
github_header="Authorization: token ${GITHUB_PR_TOKEN}"

boldprint () { printf "|\n| \033[1m${@}\033[0m\n|\n" ; }
boldcat () { printf "|\n"; while read l; do printf "| \033[1m${l}\033[0m\n"; done; printf "|\n" ; }



boldcat <<-EOT


check_tetcoin_companion_build
==============================

this job checks if there is a string in the description of the pr like

tetcoin companion: tetcoin/tetcoin#567


it will then run cargo check from this tetcoin's branch with tetcore code
from this pull request. otherwise, it will uses master instead


EOT

# Set the user name and email to make merging work
git config --global user.name 'CI system'
git config --global user.email '<>'

# Merge master into our branch before building Tetcoin to make sure we don't miss
# any commits that are required by Tetcoin.
git fetch --depth 100 origin
git merge origin/master

# Clone the current Tetcoin master branch into ./tetcoin.
# NOTE: we need to pull enough commits to be able to find a common
# ancestor for successfully performing merges below.
git clone --depth 20 https://github.com/tetcoin/tetcoin.git

cd tetcoin

# either it's a pull request then check for a companion otherwise use
# tetcoin:master
if expr match "${CI_COMMIT_REF_NAME}" '^[0-9]\+$' >/dev/null
then
  boldprint "this is pull request no ${CI_COMMIT_REF_NAME}"

  pr_data_file="$(mktemp)"
  # get the last reference to a pr in tetcoin
  curl -sSL -H "${github_header}" -o "${pr_data_file}" \
    "${github_api_tetcore_pull_url}/${CI_COMMIT_REF_NAME}"

  pr_body="$(sed -n -r 's/^[[:space:]]+"body": (".*")[^"]+$/\1/p' "${pr_data_file}")"

  pr_companion="$(echo "${pr_body}" | sed -n -r \
      -e 's;^.*[Cc]ompanion.*tetcoin/tetcoin#([0-9]+).*$;\1;p' \
      -e 's;^.*[Cc]ompanion.*https://github.com/tetcoin/tetcoin/pull/([0-9]+).*$;\1;p' \
    | tail -n 1)"

  if [ "${pr_companion}" ]
  then
    boldprint "companion pr specified/detected: #${pr_companion}"
    git fetch origin refs/pull/${pr_companion}/head:pr/${pr_companion}
    git checkout pr/${pr_companion}
    git merge origin/master
  else
    boldprint "no companion branch found - building tetcoin:master"
  fi
  rm -f "${pr_data_file}"
else
  boldprint "this is not a pull request - building tetcoin:master"
fi

cd ..
diener --tetcore --branch $CI_COMMIT_REF_NAME --git https://gitlab.tetcoin.org/tetcoin/tetcore.git --path tetcoin
cd tetcoin

# Test Tetcoin pr or master branch with this Tetcore commit.
cargo update -p tet-io
time cargo test --all --release --verbose --features=real-overseer

cd parachain/test-parachains/adder/collator/
time cargo test --release --verbose --locked --features=real-overseer
