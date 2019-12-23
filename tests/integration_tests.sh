#!/usr/bin/env bash
set -x

dir="$(dirname "$(which "$0")")"

#socat tcp-listen:10007,fork exec:cat &
#echo $! > /tmp/socat-test.pid

if test -z "$@"; then
    list="no_auth username_auth long_username_password_auth"
else
    list="$@"
fi

for test in ${list}; do
    3proxy ${dir}/${test}.cfg
    sleep 1
    cargo test --test ${test} -j1
    test_exit_code=$?

    pkill -F /tmp/3proxy-test.pid

    if test "$test_exit_code" -ne 0; then
        break
    fi
done


#pkill -F /tmp/socat-test.pid
exit ${test_exit_code}